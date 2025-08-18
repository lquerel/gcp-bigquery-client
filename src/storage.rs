//! Manage BigQuery dataset.
use futures::stream::{FuturesUnordered, Stream};
use futures::StreamExt;
use prost::Message;
use prost_types::{
    field_descriptor_proto::{Label, Type},
    DescriptorProto, FieldDescriptorProto,
};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::{collections::HashMap, convert::TryInto, fmt::Display, sync::Arc};
use tokio::pin;
use tonic::{
    transport::{Channel, ClientTlsConfig},
    Request, Streaming,
};

use crate::google::cloud::bigquery::storage::v1::{GetWriteStreamRequest, ProtoRows, WriteStream, WriteStreamView};
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
// Safety margin under 10MB to account for request metadata overhead.
const MAX_BATCH_BYTES: usize = 9 * 1024 * 1024;

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
pub struct FieldDescriptor {
    /// Field numbers starting from 1. Each subsequence field should be incremented by 1.
    pub number: u32,
    /// Field name
    pub name: String,
    /// Field type
    pub typ: ColumnType,
    /// Field mode
    pub mode: ColumnMode,
}

/// A struct to describe the schema of a table in protobuf
pub struct TableDescriptor {
    /// Descriptors of all the fields
    pub field_descriptors: Vec<FieldDescriptor>,
}

/// A struct representing a stream name
#[derive(Debug, Clone)]
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

/// A stream that yields [`AppendRowsRequest`] messages from a batch of messages.
/// Each request is guaranteed to be under 10MB in size.
pub struct AppendRequestsStream<M> {
    batch: Vec<M>,
    proto_schema: ProtoSchema,
    stream_name: StreamName,
    trace_id: String,
    current_index: usize,
}

impl<M> AppendRequestsStream<M> {
    fn new(batch: Vec<M>, proto_schema: ProtoSchema, stream_name: StreamName, trace_id: String) -> Self {
        Self {
            batch,
            proto_schema,
            stream_name,
            trace_id,
            current_index: 0,
        }
    }
}

impl<M> Stream for AppendRequestsStream<M>
where
    M: Message,
{
    type Item = AppendRowsRequest;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current_index >= self.batch.len() {
            return Poll::Ready(None);
        }

        let mut serialized_rows = Vec::new();
        let mut total_size = 0;
        let mut processed_count = 0;

        // Process messages from current_index onwards
        for msg in self.batch.iter().skip(self.current_index) {
            let encoded = msg.encode_to_vec();
            let size = encoded.len();

            if total_size + size > MAX_BATCH_BYTES && !serialized_rows.is_empty() {
                break;
            }

            serialized_rows.push(encoded);
            total_size += size;
            processed_count += 1;
        }

        if serialized_rows.is_empty() {
            return Poll::Ready(None);
        }

        let proto_rows = ProtoRows { serialized_rows };
        let proto_data = ProtoData {
            writer_schema: Some(self.proto_schema.clone()),
            rows: Some(proto_rows),
        };

        let append_rows_request = AppendRowsRequest {
            write_stream: self.stream_name.to_string(),
            offset: None,
            trace_id: self.trace_id.clone(),
            missing_value_interpretations: HashMap::new(),
            default_missing_value_interpretation: MissingValueInterpretation::Unspecified.into(),
            rows: Some(append_rows_request::Rows::ProtoRows(proto_data)),
        };

        self.current_index += processed_count;

        Poll::Ready(Some(append_rows_request))
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

    /// This function encodes the `rows` slice into a protobuf message
    /// while ensuring that the total size of the encoded rows does
    /// not exceed the `max_size` argument. The encoded rows are returned
    /// in the first value of the tuple returned by this function.
    ///
    /// Note that it is possible that not all the rows in the `rows` slice
    /// were encoded due to the `max_size` limit.  The callers can find
    /// out how many rows were processed by looking at the second value in
    /// the tuple returned by this function. If the number of rows processed
    /// is less than the number of rows in the `rows` slice, then the caller
    /// can call this function again with the rows remaing at the end of the
    /// slice to encode them.
    ///
    /// The AppendRows API has a payload size limit of 10MB. Some of the
    /// space in the 10MB limit is used by the request metadata, so the
    /// `max_size` argument should be set to a value less than 10MB. 9MB
    /// is a good value to use for the `max_size` argument.
    pub fn create_rows<M: Message>(
        table_descriptor: &TableDescriptor,
        rows: &[M],
        max_size_bytes: usize,
    ) -> (append_rows_request::Rows, usize) {
        let proto_schema = Self::create_proto_schema(table_descriptor);

        let mut serialized_rows = Vec::new();
        let mut total_size = 0;

        for row in rows {
            let encoded_row = row.encode_to_vec();
            let current_size = encoded_row.len();

            if total_size + current_size > max_size_bytes {
                break;
            }

            serialized_rows.push(encoded_row);
            total_size += current_size;
        }

        let num_rows_processed = serialized_rows.len();

        let proto_rows = ProtoRows { serialized_rows };

        let proto_data = ProtoData {
            writer_schema: Some(proto_schema),
            rows: Some(proto_rows),
        };

        (append_rows_request::Rows::ProtoRows(proto_data), num_rows_processed)
    }

    async fn new_authorized_request<T>(auth: Arc<dyn Authenticator>, message: T) -> Result<Request<T>, BQError> {
        let access_token = auth.access_token().await?;
        let bearer_token = format!("Bearer {access_token}");
        let bearer_value = bearer_token.as_str().try_into()?;

        let mut req = Request::new(message);
        let meta = req.metadata_mut();
        meta.insert("authorization", bearer_value);

        Ok(req)
    }

    fn create_field_descriptors(table_descriptor: &TableDescriptor) -> Vec<FieldDescriptorProto> {
        table_descriptor
            .field_descriptors
            .iter()
            .map(|fd| {
                let typ: Type = fd.typ.into();
                let label: Label = fd.mode.into();

                FieldDescriptorProto {
                    name: Some(fd.name.clone()),
                    number: Some(fd.number as i32),
                    label: Some(label.into()),
                    r#type: Some(typ.into()),
                    type_name: None,
                    extendee: None,
                    default_value: None,
                    oneof_index: None,
                    json_name: None,
                    options: None,
                    proto3_optional: None,
                }
            })
            .collect()
    }

    fn create_proto_descriptor(field_descriptors: Vec<FieldDescriptorProto>) -> DescriptorProto {
        DescriptorProto {
            name: Some("table_schema".to_string()),
            field: field_descriptors,
            extension: vec![],
            nested_type: vec![],
            enum_type: vec![],
            extension_range: vec![],
            oneof_decl: vec![],
            options: None,
            reserved_range: vec![],
            reserved_name: vec![],
        }
    }

    fn create_proto_schema(table_descriptor: &TableDescriptor) -> ProtoSchema {
        let field_descriptors = Self::create_field_descriptors(table_descriptor);
        let proto_descriptor = Self::create_proto_descriptor(field_descriptors);

        ProtoSchema {
            proto_descriptor: Some(proto_descriptor),
        }
    }

    /// Append rows to a table via the BigQuery Storage Write API.
    pub async fn append_rows(
        &mut self,
        stream_name: &StreamName,
        rows: append_rows_request::Rows,
        trace_id: String,
    ) -> Result<Streaming<AppendRowsResponse>, BQError> {
        let append_rows_request = AppendRowsRequest {
            write_stream: stream_name.to_string(),
            offset: None,
            trace_id,
            missing_value_interpretations: HashMap::new(),
            default_missing_value_interpretation: MissingValueInterpretation::Unspecified.into(),
            rows: Some(rows),
        };

        let req =
            Self::new_authorized_request(self.auth.clone(), tokio_stream::iter(vec![append_rows_request])).await?;

        let response = self.write_client.append_rows(req).await?;

        let streaming = response.into_inner();

        Ok(streaming)
    }

    /// Appends rows to a table using batched concurrent processing with configurable concurrency limit.
    /// For each batch in `batches`, creates a stream that yields `AppendRowsRequest` messages up to 10MB.
    /// Processes batches concurrently using `FuturesUnordered` with a maximum of `max_concurrent_batches` running simultaneously.
    pub async fn append_rows_concurrent<M>(
        &mut self,
        stream_name: &StreamName,
        table_descriptor: &TableDescriptor,
        batches: Vec<Vec<M>>,
        max_concurrent_batches: usize,
        trace_id: &str,
    ) -> Result<Vec<Vec<AppendRowsResponse>>, BQError>
    where
        M: Message + Send + 'static,
    {
        if batches.is_empty() {
            return Ok(Vec::new());
        }

        let proto_schema = Self::create_proto_schema(table_descriptor);
        let mut futures = FuturesUnordered::new();
        let mut batch = batches.into_iter();

        for _ in 0..max_concurrent_batches {
            let Some(batch) = batch.next() else {
                break;
            };

            let request_stream =
                AppendRequestsStream::new(batch, proto_schema.clone(), stream_name.clone(), trace_id.to_string());

            let client = self.clone();
            let auth = self.auth.clone();

            let future = Self::process_batch_stream(client, auth, request_stream);
            futures.push(future);
        }

        // Process completed futures and add new ones
        let mut completed_batches = Vec::new();

        while let Some(result) = futures.next().await {
            let batch_responses = result?;
            completed_batches.push(batch_responses);

            // Add a new batch if available
            if let Some(batch) = batch.next() {
                let request_stream =
                    AppendRequestsStream::new(batch, proto_schema.clone(), stream_name.clone(), trace_id.to_string());

                let client = self.clone();
                let auth = self.auth.clone();

                let future = Self::process_batch_stream(client, auth, request_stream);
                futures.push(future);
            }
        }

        let mut all_responses = Vec::with_capacity(completed_batches.len());
        for i in 0..completed_batches.len() {
            // TODO: implement this.
            // if let Some(batch_responses) = completed_batches.remove(&i) {
            //     all_responses.push(batch_responses);
            // }
        }

        Ok(all_responses)
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

        let req = Self::new_authorized_request(self.auth.clone(), get_write_stream_request).await?;

        let response = self.write_client.get_write_stream(req).await?;
        let write_stream = response.into_inner();

        Ok(write_stream)
    }

    /// Helper function to process a single batch of messages and return all responses
    async fn process_batch_stream<S>(
        mut client: StorageApi,
        auth: Arc<dyn Authenticator>,
        request_stream: S,
    ) -> Result<Vec<AppendRowsResponse>, BQError>
    where
        S: Stream<Item = AppendRowsRequest>,
    {
        let mut batch_responses = Vec::new();

        pin!(request_stream);
        while let Some(append_request) = request_stream.next().await {
            let request = Self::new_authorized_request(auth.clone(), tokio_stream::iter(vec![append_request])).await?;

            let response = client.write_client.append_rows(request).await?;
            let mut streaming = response.into_inner();

            // Drive the response stream to completion
            while let Some(response_result) = streaming.next().await {
                batch_responses.push(response_result?);
            }
        }

        Ok(batch_responses)
    }
}

#[cfg(test)]
pub mod test {
    use prost::Message;
    use std::time::{Duration, SystemTime};
    use tokio_stream::StreamExt;

    use crate::model::dataset::Dataset;
    use crate::model::field_type::FieldType;
    use crate::model::table::Table;
    use crate::model::table_field_schema::TableFieldSchema;
    use crate::model::table_schema::TableSchema;
    use crate::storage::{ColumnMode, ColumnType, FieldDescriptor, StorageApi, StreamName, TableDescriptor};
    use crate::{env_vars, Client};

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

    fn create_test_table_descriptor() -> TableDescriptor {
        let field_descriptors = vec![
            FieldDescriptor {
                name: "actor_id".to_string(),
                number: 1,
                typ: ColumnType::Int64,
                mode: ColumnMode::Nullable,
            },
            FieldDescriptor {
                name: "first_name".to_string(),
                number: 2,
                typ: ColumnType::String,
                mode: ColumnMode::Nullable,
            },
            FieldDescriptor {
                name: "last_name".to_string(),
                number: 3,
                typ: ColumnType::String,
                mode: ColumnMode::Nullable,
            },
            FieldDescriptor {
                name: "last_update".to_string(),
                number: 4,
                typ: ColumnType::String,
                mode: ColumnMode::Nullable,
            },
        ];

        TableDescriptor { field_descriptors }
    }

    async fn setup_test_table(
        client: &mut Client,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        client.dataset().delete_if_exists(project_id, dataset_id, true).await;

        let created_dataset = client.dataset().create(Dataset::new(project_id, dataset_id)).await?;
        assert_eq!(created_dataset.id, Some(format!("{project_id}:{dataset_id}")));

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

        Ok(())
    }

    fn create_test_actor(id: i32, first_name: &str) -> Actor {
        Actor {
            actor_id: id,
            first_name: first_name.to_string(),
            last_name: "Doe".to_string(),
            last_update: "2007-02-15 09:34:33 UTC".to_string(),
        }
    }

    async fn call_append_rows(
        client: &mut Client,
        table_descriptor: &TableDescriptor,
        stream_name: &StreamName,
        trace_id: String,
        mut rows: &[Actor],
        max_size: usize,
    ) -> Result<u8, Box<dyn std::error::Error>> {
        // This loop is needed because the AppendRows API has a payload size limit of 10MB and the create_rows
        // function may not process all the rows in the rows slice due to the 10MB limit. Even though in this
        // example we are only sending two rows (which won't breach the 10MB limit), in a real-world scenario,
        // we may have to send more rows and the loop will be needed to process all the rows.
        let mut num_append_rows_calls = 0;
        loop {
            let (encoded_rows, num_processed) = StorageApi::create_rows(table_descriptor, rows, max_size);
            let mut streaming = client
                .storage_mut()
                .append_rows(stream_name, encoded_rows, trace_id.clone())
                .await?;

            num_append_rows_calls += 1;

            while let Some(response) = streaming.next().await {
                response?;
            }

            // All the rows have been processed
            if num_processed == rows.len() {
                break;
            }

            // Process the remaining rows
            rows = &rows[num_processed..];
        }

        Ok(num_append_rows_calls)
    }

    #[tokio::test]
    async fn test_append_rows() {
        let (ref project_id, ref dataset_id, ref table_id, ref sa_key) = env_vars();
        let dataset_id = &format!("{dataset_id}_storage");

        let mut client = Client::from_service_account_key_file(sa_key).await.unwrap();

        setup_test_table(&mut client, project_id, dataset_id, table_id)
            .await
            .unwrap();

        let table_descriptor = create_test_table_descriptor();
        let actor1 = create_test_actor(1, "John");
        let actor2 = create_test_actor(2, "Jane");

        let stream_name = StreamName::new_default(project_id.clone(), dataset_id.clone(), table_id.clone());
        let trace_id = "test_client".to_string();

        let rows: &[Actor] = &[actor1, actor2];

        let max_size = 9 * 1024 * 1024; // 9 MB
        let num_append_rows_calls = call_append_rows(
            &mut client,
            &table_descriptor,
            &stream_name,
            trace_id.clone(),
            rows,
            max_size,
        )
        .await
        .unwrap();
        assert_eq!(num_append_rows_calls, 1);

        // It was found after experimenting that one row in this test encodes to about 38 bytes
        // We artificially limit the size of the rows to test that the loop processes all the rows
        let max_size = 50; // 50 bytes
        let num_append_rows_calls =
            call_append_rows(&mut client, &table_descriptor, &stream_name, trace_id, rows, max_size)
                .await
                .unwrap();
        assert_eq!(num_append_rows_calls, 2);
    }

    #[tokio::test]
    async fn test_append_rows_concurrent() {
        let (ref project_id, ref dataset_id, ref table_id, ref sa_key) = env_vars();
        let dataset_id = &format!("{dataset_id}_storage_limited");

        let mut client = Client::from_service_account_key_file(sa_key).await.unwrap();

        setup_test_table(&mut client, project_id, dataset_id, table_id)
            .await
            .unwrap();

        let table_descriptor = create_test_table_descriptor();
        let actor1 = create_test_actor(1, "John");
        let actor2 = create_test_actor(2, "Alex");
        let actor3 = create_test_actor(3, "Jane");
        let actor4 = create_test_actor(4, "Bob");
        let actor5 = create_test_actor(5, "Charlie");

        let stream_name = StreamName::new_default(project_id.clone(), dataset_id.clone(), table_id.clone());
        let trace_id = "test_client_limited";

        let batches = vec![
            vec![actor1.clone(), actor2.clone()],
            vec![actor3.clone()],
            vec![actor4.clone()],
            vec![actor5.clone()],
        ];

        // Test with concurrency limit of 2
        let results = client
            .storage_mut()
            .append_rows_concurrent(&stream_name, &table_descriptor, batches, 2, trace_id)
            .await
            .unwrap();

        assert_eq!(results.len(), 4);

        // Each batch should have at least one response
        for batch_responses in results {
            assert!(!batch_responses.is_empty());
            println!("Batch responses: {} responses", batch_responses.len());
        }
    }
}
