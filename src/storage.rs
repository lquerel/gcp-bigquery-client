//! Manage BigQuery dataset.
use std::{collections::HashMap, convert::TryInto, fmt::Display, sync::Arc};

use prost::Message;
use prost_types::{
    field_descriptor_proto::{Label, Type},
    DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto,
};
use tonic::{
    transport::{Channel, ClientTlsConfig},
    Request, Streaming,
};

use crate::google::cloud::bigquery::storage::v1::{GetWriteStreamRequest, WriteStream, WriteStreamView};
use crate::{
    auth::Authenticator,
    error::BQError,
    google::cloud::bigquery::storage::v1::{
        append_rows_request::{self, MissingValueInterpretation, ProtoData},
        big_query_write_client::BigQueryWriteClient,
        AppendRowsRequest, AppendRowsResponse, ProtoSchema,
    },
    BIG_QUERY_V2_URL,
};

static BIG_QUERY_STORAGE_API_URL: &str = "https://bigquerystorage.googleapis.com";
// Service Name
static BIGQUERY_STORAGE_API_DOMAIN: &str = "bigquerystorage.googleapis.com";

/// Protobuf column type
#[derive(Clone, Copy)]
pub enum ColumnType {
    Double,
    Float,
    Int64,
    Uint64,
    Int32,
    Fixed64,
    Fixed32,
    Bool,
    String,
    Bytes,
    Uint32,
    Sfixed32,
    Sfixed64,
    Sint32,
    Sint64,
    Message,
    Enum,
}

impl From<ColumnType> for Type {
    fn from(value: ColumnType) -> Self {
        match value {
            ColumnType::Double => Type::Double,
            ColumnType::Float => Type::Float,
            ColumnType::Int64 => Type::Int64,
            ColumnType::Uint64 => Type::Uint64,
            ColumnType::Int32 => Type::Int32,
            ColumnType::Fixed64 => Type::Fixed64,
            ColumnType::Fixed32 => Type::Fixed32,
            ColumnType::Bool => Type::Bool,
            ColumnType::String => Type::String,
            ColumnType::Bytes => Type::Bytes,
            ColumnType::Uint32 => Type::Uint32,
            ColumnType::Sfixed32 => Type::Sfixed32,
            ColumnType::Sfixed64 => Type::Sfixed64,
            ColumnType::Sint32 => Type::Sint32,
            ColumnType::Sint64 => Type::Sfixed64,
            ColumnType::Message => Type::Message,
            ColumnType::Enum => Type::Enum,
        }
    }
}

/// Column mode
#[derive(Clone, Copy)]
pub enum ColumnMode {
    Nullable,
    Required,
    Repeated,
}

impl From<ColumnMode> for Label {
    fn from(value: ColumnMode) -> Self {
        match value {
            ColumnMode::Nullable => Label::Optional,
            ColumnMode::Required => Label::Required,
            ColumnMode::Repeated => Label::Repeated,
        }
    }
}

/// A struct to describe the schema of a field in protobuf
#[derive(Clone)]
pub struct FieldDescriptor {
    /// Field numbers starting from 1. Each subsequence field should be incremented by 1.
    pub number: u32,

    /// Field name
    pub name: String,

    /// Field type
    pub typ: ColumnType,

    /// Field mode
    pub mode: ColumnMode,

    /// Field type name, if the field is structured or enum type
    pub type_name: Option<String>,
}

impl From<FieldDescriptor> for FieldDescriptorProto {
    fn from(fd: FieldDescriptor) -> Self {
        let typ: Type = fd.typ.into();
        let label: Label = fd.mode.into();
        Self {
            name: Some(fd.name.clone()),
            number: Some(fd.number as i32),
            label: Some(label.into()),
            r#type: Some(typ.into()),
            type_name: fd.type_name,
            extendee: None,
            default_value: None,
            oneof_index: None,
            json_name: None,
            options: None,
            proto3_optional: None,
        }
    }
}

impl FieldDescriptor {
    pub fn new<N>(name: N, number: u32, column_type: ColumnType) -> Self
    where
        N: Into<String>,
    {
        Self {
            number,
            name: name.into(),
            typ: column_type,
            mode: ColumnMode::Nullable,
            type_name: None,
        }
    }

    pub fn int64<N>(name: N, number: u32) -> Self
    where
        N: Into<String>,
    {
        Self {
            number,
            name: name.into(),
            typ: ColumnType::Int64,
            mode: ColumnMode::Nullable,
            type_name: None,
        }
    }

    pub fn uint64<N>(name: N, number: u32) -> Self
    where
        N: Into<String>,
    {
        Self {
            number,
            name: name.into(),
            typ: ColumnType::Uint64,
            mode: ColumnMode::Nullable,
            type_name: None,
        }
    }

    pub fn int32<N>(name: N, number: u32) -> Self
    where
        N: Into<String>,
    {
        Self {
            number,
            name: name.into(),
            typ: ColumnType::Int32,
            mode: ColumnMode::Nullable,
            type_name: None,
        }
    }

    pub fn uint32<N>(name: N, number: u32) -> Self
    where
        N: Into<String>,
    {
        Self {
            number,
            name: name.into(),
            typ: ColumnType::Uint32,
            mode: ColumnMode::Nullable,
            type_name: None,
        }
    }

    pub fn float<N>(name: N, number: u32) -> Self
    where
        N: Into<String>,
    {
        Self {
            number,
            name: name.into(),
            typ: ColumnType::Float,
            mode: ColumnMode::Nullable,
            type_name: None,
        }
    }

    pub fn double<N>(name: N, number: u32) -> Self
    where
        N: Into<String>,
    {
        Self {
            number,
            name: name.into(),
            typ: ColumnType::Double,
            mode: ColumnMode::Nullable,
            type_name: None,
        }
    }

    pub fn bool<N>(name: N, number: u32) -> Self
    where
        N: Into<String>,
    {
        Self {
            number,
            name: name.into(),
            typ: ColumnType::Bool,
            mode: ColumnMode::Nullable,
            type_name: None,
        }
    }

    pub fn string<N>(name: N, number: u32) -> Self
    where
        N: Into<String>,
    {
        Self {
            number,
            name: name.into(),
            typ: ColumnType::String,
            mode: ColumnMode::Nullable,
            type_name: None,
        }
    }

    pub fn message<N, TN>(name: N, number: u32, type_name: TN) -> Self
    where
        N: Into<String>,
        TN: Into<String>,
    {
        Self {
            number,
            name: name.into(),
            typ: ColumnType::Message,
            mode: ColumnMode::Nullable,
            type_name: Some(type_name.into()),
        }
    }

    pub fn bytes<N>(name: N, number: u32) -> Self
    where
        N: Into<String>,
    {
        Self {
            number,
            name: name.into(),
            typ: ColumnType::Bytes,
            mode: ColumnMode::Nullable,
            type_name: None,
        }
    }

    pub fn json<N>(name: N, number: u32) -> Self
    where
        N: Into<String>,
    {
        Self {
            number,
            name: name.into(),
            typ: ColumnType::String,
            mode: ColumnMode::Nullable,
            type_name: None,
        }
    }

    pub fn repeated<N>(name: N, number: u32, column_type: ColumnType) -> Self
    where
        N: Into<String>,
    {
        Self {
            number,
            name: name.into(),
            typ: column_type,
            mode: ColumnMode::Repeated,
            type_name: None,
        }
    }
}

/// A struct to describe the schema of a field in protobuf
#[derive(Clone)]
pub struct VariantDescriptor {
    /// Field numbers starting from 1. Each subsequence field should be incremented by 1.
    pub number: u32,

    /// Field name
    pub name: String,
}

impl From<VariantDescriptor> for EnumValueDescriptorProto {
    fn from(value: VariantDescriptor) -> Self {
        Self {
            name: Some(value.name),
            number: Some(value.number as i32),
            options: None,
        }
    }
}

/// A struct to describe the schema of a table in protobuf
pub struct TableDescriptor {
    /// Descriptors of all root fields
    pub field_descriptors: Vec<FieldDescriptor>,
    /// Descriptors of all nested types
    pub nested_descriptors: HashMap<String, Vec<FieldDescriptor>>,
    /// Descriptors of enum types
    pub enums: HashMap<String, Vec<VariantDescriptor>>,
}

impl TableDescriptor {
    /// Create a new TableDescriptor with the given root fields
    pub fn new(field_descriptors: Vec<FieldDescriptor>) -> Self {
        Self {
            field_descriptors,
            nested_descriptors: HashMap::new(),
            enums: HashMap::new(),
        }
    }
}

/// A struct representing a stream name
pub struct StreamName {
    /// Name of the project
    project: String,

    /// Name of the dataset
    dataset: String,

    /// Name of the table
    table: String,

    /// Name of the stream
    stream: String,
}

impl StreamName {
    pub fn new(project: String, dataset: String, table: String, stream: String) -> StreamName {
        StreamName {
            project,
            dataset,
            table,
            stream,
        }
    }

    pub fn new_default(project: String, dataset: String, table: String) -> StreamName {
        StreamName {
            project,
            dataset,
            table,
            stream: "_default".to_string(),
        }
    }
}

impl Display for StreamName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let StreamName {
            project,
            dataset,
            table,
            stream,
        } = self;
        f.write_fmt(format_args!(
            "projects/{project}/datasets/{dataset}/tables/{table}/streams/{stream}"
        ))
    }
}

/// A dataset API handler.
#[derive(Clone)]
pub struct StorageApi {
    write_client: BigQueryWriteClient<Channel>,
    auth: Arc<dyn Authenticator>,
    base_url: String,
}

impl StorageApi {
    pub(crate) fn new(write_client: BigQueryWriteClient<Channel>, auth: Arc<dyn Authenticator>) -> Self {
        Self {
            write_client,
            auth,
            base_url: BIG_QUERY_V2_URL.to_string(),
        }
    }

    pub(crate) async fn new_write_client() -> Result<BigQueryWriteClient<Channel>, BQError> {
        // Since Tonic 0.12.0, TLS root certificates are no longer implicit.
        // We need to specify them explicitly.
        // See: https://github.com/hyperium/tonic/pull/1731
        let tls_config = ClientTlsConfig::new()
            .domain_name(BIGQUERY_STORAGE_API_DOMAIN)
            .with_native_roots();
        let channel = Channel::from_static(BIG_QUERY_STORAGE_API_URL)
            .tls_config(tls_config)?
            .connect()
            .await?;
        let write_client = BigQueryWriteClient::new(channel);

        Ok(write_client)
    }

    pub(crate) fn with_base_url(&mut self, base_url: String) -> &mut Self {
        self.base_url = base_url;
        self
    }

    /// Append rows to a table via the BigQuery Storage Write API.
    pub async fn append_rows<M: Message>(
        &mut self,
        stream_name: &StreamName,
        table_descriptor: &TableDescriptor,
        rows: &[M],
        trace_id: String,
    ) -> Result<Streaming<AppendRowsResponse>, BQError> {
        let write_stream = stream_name.to_string();

        let append_rows_request = AppendRowsRequest {
            write_stream,
            offset: None,
            trace_id,
            missing_value_interpretations: HashMap::new(),
            default_missing_value_interpretation: MissingValueInterpretation::Unspecified.into(),
            rows: Some(Self::create_rows(table_descriptor, rows)),
        };

        let req = self
            .new_authorized_request(tokio_stream::iter(vec![append_rows_request]))
            .await?;

        let response = self.write_client.append_rows(req).await?;

        let streaming = response.into_inner();

        Ok(streaming)
    }

    fn create_rows<M: Message>(table_descriptor: &TableDescriptor, rows: &[M]) -> append_rows_request::Rows {
        let field_descriptors = table_descriptor
            .field_descriptors
            .iter()
            .cloned()
            .map(FieldDescriptorProto::from)
            .collect();
        let nested_descriptors = table_descriptor
            .nested_descriptors
            .iter()
            .map(|(name, fds)| DescriptorProto {
                name: Some(name.to_owned()),
                field: fds.iter().cloned().map(FieldDescriptorProto::from).collect(),
                extension: vec![],
                nested_type: vec![],
                enum_type: vec![],
                extension_range: vec![],
                oneof_decl: vec![],
                options: None,
                reserved_range: vec![],
                reserved_name: vec![],
            })
            .collect();
        let enums = table_descriptor
            .enums
            .iter()
            .map(|(name, variants)| EnumDescriptorProto {
                name: Some(name.clone()),
                value: variants.iter().cloned().map(EnumValueDescriptorProto::from).collect(),
                options: None,
                reserved_range: vec![],
                reserved_name: vec![],
            })
            .collect();
        let proto_descriptor = DescriptorProto {
            name: Some("table_schema".to_string()),
            field: field_descriptors,
            extension: vec![],
            nested_type: nested_descriptors,
            enum_type: enums,
            extension_range: vec![],
            oneof_decl: vec![],
            options: None,
            reserved_range: vec![],
            reserved_name: vec![],
        };
        let proto_schema = ProtoSchema {
            proto_descriptor: Some(proto_descriptor),
        };

        let rows = rows.iter().map(|m| m.encode_to_vec()).collect();

        let proto_rows = crate::google::cloud::bigquery::storage::v1::ProtoRows { serialized_rows: rows };

        let proto_data = ProtoData {
            writer_schema: Some(proto_schema),
            rows: Some(proto_rows),
        };
        append_rows_request::Rows::ProtoRows(proto_data)
    }

    async fn new_authorized_request<D>(&self, t: D) -> Result<Request<D>, BQError> {
        let access_token = self.auth.access_token().await?;
        let bearer_token = format!("Bearer {access_token}");
        let bearer_value = bearer_token.as_str().try_into()?;
        let mut req = Request::new(t);
        let meta = req.metadata_mut();
        meta.insert("authorization", bearer_value);
        Ok(req)
    }

    pub async fn get_write_stream(
        &mut self,
        stream_name: &StreamName,
        view: WriteStreamView,
    ) -> Result<WriteStream, BQError> {
        let get_write_stream_request = GetWriteStreamRequest {
            name: stream_name.to_string(),
            view: view.into(),
        };

        let req = self.new_authorized_request(get_write_stream_request).await?;

        let response = self.write_client.get_write_stream(req).await?;
        let write_stream = response.into_inner();

        Ok(write_stream)
    }
}

#[cfg(test)]
pub mod test {
    use crate::model::dataset::Dataset;
    use crate::model::field_type::FieldType;
    use crate::model::table::Table;
    use crate::model::table_field_schema::TableFieldSchema;
    use crate::model::table_schema::TableSchema;
    use crate::storage::{FieldDescriptor, StreamName, TableDescriptor};
    use crate::{env_vars, Client};
    use prost::Message;
    use std::time::{Duration, SystemTime};
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn test() -> Result<(), Box<dyn std::error::Error>> {
        let (ref project_id, ref dataset_id, ref table_id, ref sa_key) = env_vars();
        let dataset_id = &format!("{dataset_id}_storage");

        let mut client = Client::from_service_account_key_file(sa_key).await?;

        // Delete the dataset if needed
        client.dataset().delete_if_exists(project_id, dataset_id, true).await;

        // Create dataset
        let created_dataset = client.dataset().create(Dataset::new(project_id, dataset_id)).await?;
        assert_eq!(created_dataset.id, Some(format!("{project_id}:{dataset_id}")));

        // Create table
        let table = Table::new(
            project_id,
            dataset_id,
            table_id,
            TableSchema::new(vec![
                TableFieldSchema::new("actor_id", FieldType::Int64),
                TableFieldSchema::new("first_name", FieldType::String),
                TableFieldSchema::new("last_name", FieldType::String),
                TableFieldSchema::new("last_update", FieldType::Timestamp),
            ]),
        );
        let created_table = client
            .table()
            .create(
                table
                    .description("A table used for unit tests")
                    .label("owner", "me")
                    .label("env", "prod")
                    .expiration_time(SystemTime::now() + Duration::from_secs(3600)),
            )
            .await?;
        assert_eq!(created_table.table_reference.table_id, table_id.to_string());

        // let (ref project_id, ref dataset_id, ref table_id, ref gcp_sa_key) = env_vars();
        //
        // let mut client = crate::Client::from_service_account_key_file(gcp_sa_key).await?;

        let field_descriptors = vec![
            FieldDescriptor::int64("actor_id".to_string(), 1),
            FieldDescriptor::string("first_name".to_string(), 2),
            FieldDescriptor::string("last_name".to_string(), 3),
            FieldDescriptor::string("last_update".to_string(), 4),
        ];
        let table_descriptor = TableDescriptor::new(field_descriptors);

        #[derive(Clone, PartialEq, Message)]
        struct Actor {
            #[prost(int32, tag = "1")]
            actor_id: i32,

            #[prost(string, tag = "2")]
            first_name: String,

            #[prost(string, tag = "3")]
            last_name: String,

            #[prost(string, tag = "4")]
            last_update: String,
        }

        let actor1 = Actor {
            actor_id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            last_update: "2007-02-15 09:34:33 UTC".to_string(),
        };

        let actor2 = Actor {
            actor_id: 2,
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
            last_update: "2008-02-15 09:34:33 UTC".to_string(),
        };

        let stream_name = StreamName::new_default(project_id.clone(), dataset_id.clone(), table_id.clone());
        let trace_id = "test_client".to_string();

        let mut streaming = client
            .storage_mut()
            .append_rows(&stream_name, &table_descriptor, &[actor1, actor2], trace_id)
            .await?;

        while let Some(resp) = streaming.next().await {
            let resp = resp?;
            println!("response: {resp:#?}");
        }

        Ok(())
    }
}
