//! BigQuery Storage Write API client and utilities.
//!
//! Provides functionality for appending rows to BigQuery tables using the
//! Storage Write API, which offers higher throughput than the traditional
//! BigQuery API for streaming inserts.

use futures::stream::Stream;
use futures::StreamExt;
use pin_project::pin_project;
use prost::Message;
use prost_types::{
    field_descriptor_proto::{Label, Type},
    DescriptorProto, FieldDescriptorProto,
};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::{collections::HashMap, convert::TryInto, fmt::Display, sync::Arc};
use tokio::pin;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tonic::{
    transport::{Channel, ClientTlsConfig},
    Request, Status, Streaming,
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

/// Base URL for the BigQuery Storage Write API endpoint.
static BIG_QUERY_STORAGE_API_URL: &str = "https://bigquerystorage.googleapis.com";
/// Domain name for BigQuery Storage API used in TLS configuration.
static BIGQUERY_STORAGE_API_DOMAIN: &str = "bigquerystorage.googleapis.com";
/// Maximum size limit for batched append requests in bytes.
///
/// Set to 9MB to provide safety margin under the 10MB BigQuery API limit,
/// accounting for request metadata overhead.
const MAX_BATCH_BYTES: usize = 9 * 1024 * 1024;

/// Supported protobuf column types for BigQuery schema mapping.
#[derive(Debug, Copy, Clone)]
pub enum ColumnType {
    /// 64-bit floating point number.
    Double,
    /// 32-bit floating point number.
    Float,
    /// 64-bit signed integer.
    Int64,
    /// 64-bit unsigned integer.
    Uint64,
    /// 32-bit signed integer.
    Int32,
    /// 64-bit fixed-width unsigned integer.
    Fixed64,
    /// 32-bit fixed-width unsigned integer.
    Fixed32,
    /// Boolean value.
    Bool,
    /// UTF-8 encoded string.
    String,
    /// Arbitrary byte sequence.
    Bytes,
    /// 32-bit unsigned integer.
    Uint32,
    /// 32-bit signed fixed-width integer.
    Sfixed32,
    /// 64-bit signed fixed-width integer.
    Sfixed64,
    /// 32-bit signed integer with zigzag encoding.
    Sint32,
    /// 64-bit signed integer with zigzag encoding.
    Sint64,
}

/// Converts [`ColumnType`] to protobuf [`Type`] enum.
impl From<ColumnType> for Type {
    /// Maps column type to corresponding protobuf type identifier.
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

/// Field cardinality modes for BigQuery schema fields.
#[derive(Debug, Copy, Clone)]
pub enum ColumnMode {
    /// Field may contain null values.
    Nullable,
    /// Field must always contain a value.
    Required,
    /// Field contains an array of values.
    Repeated,
}

/// Converts [`ColumnMode`] to protobuf [`Label`] enum.
impl From<ColumnMode> for Label {
    /// Maps column mode to corresponding protobuf label.
    fn from(value: ColumnMode) -> Self {
        match value {
            ColumnMode::Nullable => Label::Optional,
            ColumnMode::Required => Label::Required,
            ColumnMode::Repeated => Label::Repeated,
        }
    }
}

/// Schema descriptor for a single field in a BigQuery table.
///
/// Contains all metadata needed to define a field in the protobuf schema
/// used by the BigQuery Storage Write API.
#[derive(Debug)]
pub struct FieldDescriptor {
    /// Unique field number starting from 1, incrementing for each field.
    pub number: u32,
    /// Name of the field as it appears in BigQuery.
    pub name: String,
    /// Data type of the field.
    pub typ: ColumnType,
    /// Cardinality mode of the field.
    pub mode: ColumnMode,
}

/// Complete schema descriptor for a BigQuery table.
///
/// Contains field descriptors for all columns in the table.
#[derive(Debug)]
pub struct TableDescriptor {
    /// Collection of field descriptors defining the table schema.
    pub field_descriptors: Vec<FieldDescriptor>,
}

/// Fully qualified identifier for a BigQuery write stream.
///
/// Encapsulates the hierarchical naming structure used by BigQuery
/// to identify tables and their associated write streams.
#[derive(Debug, Clone)]
pub struct StreamName {
    /// Google Cloud project identifier.
    project: String,
    /// BigQuery dataset identifier within the project.
    dataset: String,
    /// BigQuery table identifier within the dataset.
    table: String,
    /// Write stream identifier for the table.
    stream: String,
}

impl StreamName {
    /// Creates a new stream name with custom stream identifier.
    pub fn new(project: String, dataset: String, table: String, stream: String) -> StreamName {
        StreamName {
            project,
            dataset,
            table,
            stream,
        }
    }

    /// Creates a new stream name using the default stream identifier.
    ///
    /// Uses "_default" as the stream name, which is the standard
    /// stream for most BigQuery write operations.
    pub fn new_default(project: String, dataset: String, table: String) -> StreamName {
        StreamName {
            project,
            dataset,
            table,
            stream: "_default".to_string(),
        }
    }
}

/// Formats [`StreamName`] as a BigQuery-compatible resource path.
impl Display for StreamName {
    /// Formats the stream name as a fully qualified resource path.
    ///
    /// Returns a string in the format:
    /// `projects/{project}/datasets/{dataset}/tables/{table}/streams/{stream}`
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

/// Stream that converts batched messages into [`AppendRowsRequest`] objects.
///
/// Automatically splits large batches into multiple requests, each staying
/// under the 10MB BigQuery API limit. Implements [`Stream`] for use with
/// async streaming APIs.
#[pin_project]
#[derive(Debug)]
pub struct AppendRequestsStream<M> {
    /// Collection of messages to be converted into append requests.
    #[pin]
    batch: Vec<M>,
    /// Protobuf schema definition for the target table.
    proto_schema: ProtoSchema,
    /// Target stream identifier for the append operations.
    stream_name: StreamName,
    /// Unique identifier for tracing and debugging requests.
    trace_id: String,
    /// Current position in the batch being processed.
    current_index: usize,
}

impl<M> AppendRequestsStream<M> {
    /// Creates a new append requests stream from a batch of messages.
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

/// Implements streaming functionality for message batches.
///
/// Yields [`AppendRowsRequest`] objects that respect the BigQuery API
/// size limits while preserving message ordering.
impl<M> Stream for AppendRequestsStream<M>
where
    M: Message,
{
    type Item = AppendRowsRequest;

    /// Polls for the next append request from the batch.
    ///
    /// Returns [`Poll::Ready(None)`] when all messages have been processed.
    /// Each returned request contains as many messages as possible while
    /// staying under the size limit.
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        if *this.current_index >= this.batch.len() {
            return Poll::Ready(None);
        }

        let mut serialized_rows = Vec::new();
        let mut total_size = 0;
        let mut processed_count = 0;

        // Process messages from current_index onwards
        for msg in this.batch.iter().skip(*this.current_index) {
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
            writer_schema: Some(this.proto_schema.clone()),
            rows: Some(proto_rows),
        };

        let append_rows_request = AppendRowsRequest {
            write_stream: this.stream_name.to_string(),
            offset: None,
            trace_id: this.trace_id.clone(),
            missing_value_interpretations: HashMap::new(),
            default_missing_value_interpretation: MissingValueInterpretation::Unspecified.into(),
            rows: Some(append_rows_request::Rows::ProtoRows(proto_data)),
        };

        *this.current_index += processed_count;

        Poll::Ready(Some(append_rows_request))
    }
}

/// Client for BigQuery Storage Write API operations.
///
/// Provides methods for appending rows to BigQuery tables using the
/// high-throughput Storage Write API. Handles authentication, request
/// batching, and concurrent processing.
#[derive(Clone)]
pub struct StorageApi {
    /// gRPC client for BigQuery Storage Write API.
    write_client: BigQueryWriteClient<Channel>,
    /// Authentication provider for API requests.
    auth: Arc<dyn Authenticator>,
    /// Base URL for BigQuery API endpoints.
    base_url: String,
}

impl StorageApi {
    /// Creates a new storage API client with the provided components.
    pub(crate) fn new(write_client: BigQueryWriteClient<Channel>, auth: Arc<dyn Authenticator>) -> Self {
        Self {
            write_client,
            auth,
            base_url: BIG_QUERY_V2_URL.to_string(),
        }
    }

    /// Creates a new gRPC client for the BigQuery Storage Write API.
    ///
    /// Establishes a TLS connection to the BigQuery Storage API endpoint
    /// with proper certificate validation.
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

    /// Sets a custom base URL for BigQuery API endpoints.
    ///
    /// Used primarily for testing with mock endpoints.
    pub(crate) fn with_base_url(&mut self, base_url: String) -> &mut Self {
        self.base_url = base_url;
        self
    }

    /// Encodes message rows into protobuf format with size constraints.
    ///
    /// Processes as many rows as possible while staying under the specified
    /// size limit. Returns the encoded data and the number of rows processed.
    /// Callers should check the returned count to determine if additional
    /// calls are needed for remaining rows.
    ///
    /// The `max_size_bytes` should be less than 10MB to account for request
    /// metadata overhead. 9MB is recommended.
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

    /// Creates an authorized gRPC request with Bearer token authentication.
    async fn new_authorized_request<T>(auth: Arc<dyn Authenticator>, message: T) -> Result<Request<T>, BQError> {
        let access_token = auth.access_token().await?;
        let bearer_token = format!("Bearer {access_token}");
        let bearer_value = bearer_token.as_str().try_into()?;

        let mut request = Request::new(message);
        let meta = request.metadata_mut();
        meta.insert("authorization", bearer_value);

        Ok(request)
    }

    /// Converts table field descriptors to protobuf field descriptor format.
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

    /// Creates a protobuf descriptor proto from field descriptors.
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

    /// Generates a complete protobuf schema from table descriptor.
    fn create_proto_schema(table_descriptor: &TableDescriptor) -> ProtoSchema {
        let field_descriptors = Self::create_field_descriptors(table_descriptor);
        let proto_descriptor = Self::create_proto_descriptor(field_descriptors);

        ProtoSchema {
            proto_descriptor: Some(proto_descriptor),
        }
    }

    /// Retrieves metadata for a BigQuery write stream.
    ///
    /// Returns stream information including schema and state details
    /// based on the requested view level.
    pub async fn get_write_stream(
        &mut self,
        stream_name: &StreamName,
        view: WriteStreamView,
    ) -> Result<WriteStream, BQError> {
        let get_write_stream_request = GetWriteStreamRequest {
            name: stream_name.to_string(),
            view: view.into(),
        };

        let request = Self::new_authorized_request(self.auth.clone(), get_write_stream_request).await?;
        let response = self.write_client.get_write_stream(request).await?;
        let write_stream = response.into_inner();

        Ok(write_stream)
    }

    /// Appends rows to a BigQuery table using the Storage Write API.
    ///
    /// Sends the provided rows to the specified stream and returns a
    /// streaming response containing the results.
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

        let request =
            Self::new_authorized_request(self.auth.clone(), tokio_stream::iter(vec![append_rows_request])).await?;
        let response = self.write_client.append_rows(request).await?;
        let streaming = response.into_inner();

        Ok(streaming)
    }

    /// Appends multiple batches of rows concurrently with controlled parallelism.
    ///
    /// Processes batches in parallel up to the specified concurrency limit.
    /// Each batch is converted into appropriately sized requests and responses
    /// are collected and returned in the same order as input batches.
    /// Concurrency slots are held until each batch's response stream is
    /// fully consumed.
    pub async fn append_rows_concurrent<M>(
        &mut self,
        stream_name: &StreamName,
        table_descriptor: &TableDescriptor,
        batches: Vec<Vec<M>>,
        max_concurrent_batches: usize,
        trace_id: &str,
    ) -> Result<Vec<(usize, Vec<Result<AppendRowsResponse, Status>>)>, BQError>
    where
        M: Message + Send + 'static,
    {
        if batches.is_empty() {
            return Ok(Vec::new());
        }

        let batches_num = batches.len();
        let proto_schema = Self::create_proto_schema(table_descriptor);
        let semaphore = Arc::new(Semaphore::new(max_concurrent_batches));

        let mut join_set = JoinSet::new();
        for (idx, batch) in batches.into_iter().enumerate() {
            // Acquire a concurrency slot and hold it until responses are fully drained.
            let permit = semaphore.clone().acquire_owned().await?;

            let stream_name = stream_name.clone();
            let trace_id = trace_id.to_string();
            let proto_schema = proto_schema.clone();
            let mut client = self.clone();

            join_set.spawn(async move {
                // Build the request stream for this batch.
                let request_stream = AppendRequestsStream::new(batch, proto_schema, stream_name, trace_id);

                let mut responses = Vec::new();
                match Self::new_authorized_request(client.auth.clone(), request_stream).await {
                    Ok(request) => match client.write_client.append_rows(request).await {
                        Ok(response) => {
                            let mut streaming_response = response.into_inner();
                            while let Some(response) = streaming_response.next().await {
                                responses.push(response);
                            }
                        }
                        Err(status) => {
                            responses.push(Err(status));
                        }
                    },
                    Err(err) => {
                        responses.push(Err(Status::unknown(err.to_string())));
                    }
                }

                // Free the concurrency slot only after fully draining the responses or after an error.
                drop(permit);

                (idx, responses)
            });
        }

        // Collect all task results in the order of completion, we do not care about the order of the batches.
        let mut responses = Vec::with_capacity(batches_num);
        while let Some(response) = join_set.join_next().await {
            let (idx, response) = response?;
            responses.push((idx, response));
        }

        Ok(responses)
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
        let batch_responses = client
            .storage_mut()
            .append_rows_concurrent(&stream_name, &table_descriptor, batches, 2, trace_id)
            .await
            .unwrap();

        assert_eq!(batch_responses.len(), 4);
    }
}
