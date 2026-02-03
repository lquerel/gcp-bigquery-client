//! BigQuery Storage Write API client for high-throughput data streaming.
//!
//! This module provides an implementation of the BigQuery Storage Write API,
//! enabling efficient streaming of structured data to BigQuery tables.

use deadpool::managed::{Manager, Object, Pool, QueueMode};
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
use std::time::Duration;
use std::{
    collections::HashMap,
    convert::TryInto,
    fmt::Display,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tonic::{
    codec::CompressionEncoding,
    transport::{Channel, ClientTlsConfig},
    Request, Status, Streaming,
};
use tracing::{debug, info, warn};

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
const MAX_BATCH_SIZE_BYTES: usize = 9 * 1024 * 1024;
/// Maximum message size for tonic gRPC client configuration.
///
/// Set to 20MB to accommodate large response messages and provide headroom
/// for metadata while staying within reasonable memory bounds.
const MAX_MESSAGE_SIZE_BYTES: usize = 20 * 1024 * 1024;
/// The name of the default stream in BigQuery.
///
/// This stream is a special built-in stream that always exists for a table.
const DEFAULT_STREAM_NAME: &str = "_default";
/// Default number of connections in the pool.
///
/// With HTTP/2 multiplexing, a small number of connections (2-4) is sufficient
/// for high throughput since multiple gRPC streams share each connection.
/// Google recommends reusing connections extensively—a single connection can
/// support 1-10+ MBps throughput. Multiple connections provide fault isolation
/// rather than increased parallelism.
const DEFAULT_POOL_SIZE: usize = 4;

/// Default maximum inflight requests per connection.
///
/// HTTP/2 connections can handle many concurrent streams efficiently. Google
/// recommends up to 100 concurrent streams per connection for optimal throughput.
const DEFAULT_REQUESTS_PER_CONNECTION: usize = 100;
/// HTTP/2 keepalive interval in seconds.
///
/// Sends PING frames at this interval to keep connections alive and detect
/// dead connections. Set to 30 seconds to stay well under typical proxy
/// idle timeouts (GCP: 10 min, AWS ELB: 60 sec).
const HTTP2_KEEPALIVE_INTERVAL_SECS: u64 = 30;
/// HTTP/2 keepalive timeout in seconds.
///
/// If a PING is not acknowledged within this time, the connection is
/// considered dead.
const HTTP2_KEEPALIVE_TIMEOUT_SECS: u64 = 10;

/// Configuration for the BigQuery Storage Write API client.
///
/// A single HTTP/2 connection can support 1-10+ MBps throughput with up to
/// 100 concurrent gRPC streams. The pool provides fault isolation rather than
/// increased parallelism.
#[derive(Debug, Clone)]
pub struct StorageApiConfig {
    /// Number of connections in the pool.
    ///
    /// With HTTP/2 multiplexing, a small number of connections (2-4) is typically
    /// sufficient for high throughput. Multiple gRPC streams share each connection,
    /// so additional connections provide fault isolation rather than parallelism.
    /// Default: 4.
    pub pool_size: usize,
    /// Maximum number of inflight gRPC requests across all StorageApi operations.
    ///
    /// This is a global limit shared across all concurrent calls to the StorageApi.
    /// Each request acquires a permit from a shared semaphore before making a gRPC
    /// call. Google recommends up to 100 concurrent streams per connection.
    /// Default: 400 (pool_size × 100 requests per connection).
    pub max_inflight_requests: usize,
}

impl Default for StorageApiConfig {
    fn default() -> Self {
        Self {
            pool_size: DEFAULT_POOL_SIZE,
            max_inflight_requests: DEFAULT_POOL_SIZE * DEFAULT_REQUESTS_PER_CONNECTION,
        }
    }
}

impl StorageApiConfig {
    /// Creates a new configuration with the specified pool size.
    pub fn with_pool_size(mut self, pool_size: usize) -> Self {
        self.pool_size = pool_size;
        self
    }

    /// Creates a new configuration with the specified max inflight requests.
    pub fn with_max_inflight_requests(mut self, max_inflight_requests: usize) -> Self {
        self.max_inflight_requests = max_inflight_requests;
        self
    }
}

/// Connection pool for managing multiple gRPC clients to BigQuery Storage Write API.
///
/// Uses deadpool to maintain a pool of persistent connections with proper
/// resource management, automatic cleanup, and efficient connection reuse.
#[derive(Clone)]
struct ConnectionPool(Pool<BigQueryWriteClientManager>);

/// Wrapper around BigQueryWriteClient.
struct PooledClient {
    /// The underlying gRPC client.
    client: BigQueryWriteClient<Channel>,
}

/// Manager for creating and managing BigQuery Storage Write API gRPC clients.
///
/// Implements the deadpool Manager trait to handle connection lifecycle
/// including creation, health checks, and cleanup of gRPC clients.
struct BigQueryWriteClientManager;

impl Manager for BigQueryWriteClientManager {
    type Type = PooledClient;
    type Error = BQError;

    /// Creates a new gRPC client for BigQuery Storage Write API.
    ///
    /// Establishes a secure TLS connection with HTTP/2 keepalive, compression,
    /// and size limits configured for optimal performance and reliability.
    async fn create(&self) -> Result<Self::Type, Self::Error> {
        // Since Tonic 0.12.0, TLS root certificates are no longer included by default.
        // They must now be specified explicitly.
        // See: https://github.com/hyperium/tonic/pull/1731
        let tls_config = ClientTlsConfig::new()
            .domain_name(BIGQUERY_STORAGE_API_DOMAIN)
            .with_enabled_roots();

        let channel = Channel::from_static(BIG_QUERY_STORAGE_API_URL)
            .tls_config(tls_config)?
            // Configure HTTP/2 keepalive to detect dead connections and prevent
            // proxies from closing idle connections.
            .http2_keep_alive_interval(Duration::from_secs(HTTP2_KEEPALIVE_INTERVAL_SECS))
            .keep_alive_timeout(Duration::from_secs(HTTP2_KEEPALIVE_TIMEOUT_SECS))
            .keep_alive_while_idle(true)
            .connect()
            .await?;

        let client = BigQueryWriteClient::new(channel)
            .max_encoding_message_size(MAX_MESSAGE_SIZE_BYTES)
            .max_decoding_message_size(MAX_MESSAGE_SIZE_BYTES)
            .send_compressed(CompressionEncoding::Gzip)
            .accept_compressed(CompressionEncoding::Gzip);

        debug!("grpc connection created");

        Ok(PooledClient { client })
    }

    /// Recycles a gRPC client connection back into the pool.
    ///
    /// With HTTP/2 keepalive enabled, dead connections are detected automatically
    /// via PING frames. Age-based recycling is unnecessary and conflicts with
    /// Google's recommendation to reuse connections for as many writes as possible.
    async fn recycle(
        &self,
        _conn: &mut Self::Type,
        _metrics: &deadpool::managed::Metrics,
    ) -> deadpool::managed::RecycleResult<Self::Error> {
        Ok(())
    }
}

impl ConnectionPool {
    /// Creates a new connection pool with the specified maximum size.
    ///
    /// Establishes a managed pool that creates connections on-demand and
    /// recycles them efficiently for optimal performance. With HTTP/2
    /// multiplexing, a small number of connections (2-4) is typically
    /// sufficient for high throughput, as multiple gRPC streams can
    /// share the same underlying connection.
    async fn new(max_size: usize) -> Result<Self, BQError> {
        let manager = BigQueryWriteClientManager;
        let pool = Pool::builder(manager)
            .max_size(max_size)
            // We must use Fifo since we want to always get the least recently used connection to cycle
            // through connections in the pool.
            .queue_mode(QueueMode::Fifo)
            .build()
            .map_err(|e| BQError::ConnectionPoolError(format!("failed to create connection pool: {e}")))?;

        debug!(max_size, "connection pool initialized");
        Ok(Self(pool))
    }

    /// Retrieves a client from the pool.
    ///
    /// Returns a managed connection object that automatically returns
    /// the connection to the pool when dropped.
    async fn get_client(&self) -> Result<Object<BigQueryWriteClientManager>, BQError> {
        self.0
            .get()
            .await
            .map_err(|e| BQError::ConnectionPoolError(format!("failed to get connection from pool: {e}")))
    }

    /// Releases all connections currently held in the pool.
    ///
    /// Removes all idle connections, forcing new requests to create fresh
    /// connections. Useful for recovering from network errors or refreshing
    /// stale connections.
    fn release_all(&self) {
        self.0.retain(|_, _| false);
        debug!("all connections released from pool");
    }
}

/// Supported protobuf column types for BigQuery schema mapping.
#[derive(Debug, Copy, Clone)]
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
    /// Converts [`ColumnType`] to protobuf [`Type`] enum value.
    ///
    /// Maps each column type variant to its corresponding protobuf type
    /// identifier used in BigQuery Storage Write API schema definitions.
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
            ColumnType::Sint64 => Type::Sint64,
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

impl From<ColumnMode> for Label {
    /// Converts [`ColumnMode`] to protobuf [`Label`] enum value.
    ///
    /// Maps field cardinality modes to their corresponding protobuf labels
    /// used in BigQuery Storage Write API schema definitions.
    fn from(value: ColumnMode) -> Self {
        match value {
            ColumnMode::Nullable => Label::Optional,
            ColumnMode::Required => Label::Required,
            ColumnMode::Repeated => Label::Repeated,
        }
    }
}

/// Metadata descriptor for a single field in a BigQuery table schema.
///
/// Contains the complete field definition including data type, cardinality mode,
/// and protobuf field number required for BigQuery Storage Write API operations.
/// Each field descriptor maps directly to a protobuf field descriptor in the
/// generated schema.
#[derive(Debug, Clone)]
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

/// Complete schema definition for a BigQuery table.
///
/// Aggregates all field descriptors that define the table's structure.
/// Used to generate protobuf schemas for BigQuery Storage Write API operations
/// and validate row data before transmission.
#[derive(Debug, Clone)]
pub struct TableDescriptor {
    /// Collection of field descriptors defining the table schema.
    pub field_descriptors: Vec<FieldDescriptor>,
}

/// Internal storage for [`TableBatch`] data.
#[derive(Debug)]
struct TableBatchInner<M> {
    stream_name: StreamName,
    table_descriptor: TableDescriptor,
    rows: Vec<M>,
}

/// Collection of rows targeting a specific BigQuery table for batch processing.
///
/// Encapsulates rows with their destination stream and schema metadata,
/// enabling efficient batch operations and optimal parallelism distribution
/// across multiple tables in concurrent append operations. Cloning is cheap
/// as the data is stored behind an [`Arc`].
#[derive(Debug, Clone)]
pub struct TableBatch<M>(Arc<TableBatchInner<M>>);

impl<M> TableBatch<M> {
    /// Creates a new table batch targeting the specified stream.
    ///
    /// Combines rows with their destination metadata to form a complete
    /// batch ready for processing by append operations.
    pub fn new(stream_name: StreamName, table_descriptor: TableDescriptor, rows: Vec<M>) -> Self {
        Self(Arc::new(TableBatchInner {
            stream_name,
            table_descriptor,
            rows,
        }))
    }

    /// Returns the target stream name.
    pub fn stream_name(&self) -> &StreamName {
        &self.0.stream_name
    }

    /// Returns the table descriptor.
    pub fn table_descriptor(&self) -> &TableDescriptor {
        &self.0.table_descriptor
    }

    /// Returns the rows in this batch.
    pub fn rows(&self) -> &[M] {
        &self.0.rows
    }
}

/// Result of processing a single table batch in concurrent append operations.
///
/// Contains the batch processing results along with metadata about the operation,
/// including the original batch index for result ordering and total bytes sent
/// for monitoring and debugging purposes.
#[derive(Debug)]
pub struct BatchAppendResult {
    /// Original index of the batch in the input vector.
    ///
    /// Allows callers to correlate results with their original batch ordering
    /// even when results are returned in completion order rather than submission order.
    pub batch_index: usize,
    /// Collection of append operation responses for this batch.
    ///
    /// Each batch may generate multiple append requests due to size limits,
    /// resulting in multiple responses. All responses must be checked for
    /// errors to ensure complete batch success.
    pub responses: Vec<Result<AppendRowsResponse, Status>>,
    /// Total bytes sent for this batch across all requests.
    pub bytes_sent: usize,
}

impl BatchAppendResult {
    /// Creates a new batch append result.
    ///
    /// Combines all result metadata into a single cohesive structure
    /// for easier handling by calling code.
    pub fn new(batch_index: usize, responses: Vec<Result<AppendRowsResponse, Status>>, bytes_sent: usize) -> Self {
        Self {
            batch_index,
            responses,
            bytes_sent,
        }
    }

    /// Returns true if all responses in this batch are successful.
    ///
    /// Convenience method to quickly check batch success without
    /// iterating through individual responses.
    pub fn is_success(&self) -> bool {
        self.responses.iter().all(|result| result.is_ok())
    }
}

/// Hierarchical identifier for BigQuery write streams.
///
/// Represents the complete resource path structure used by BigQuery to
/// uniquely identify tables and their associated write streams within
/// the Google Cloud resource hierarchy.
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
    /// Creates a stream name with all components specified.
    ///
    /// Constructs a fully qualified stream identifier using custom
    /// project, dataset, table, and stream components.
    pub fn new(project: String, dataset: String, table: String, stream: String) -> StreamName {
        StreamName {
            project,
            dataset,
            table,
            stream,
        }
    }

    /// Creates a stream name using the default stream identifier.
    ///
    /// Uses "_default" as the stream component, which is the standard
    /// stream identifier for most BigQuery write operations.
    pub fn new_default(project: String, dataset: String, table: String) -> StreamName {
        StreamName {
            project,
            dataset,
            table,
            stream: DEFAULT_STREAM_NAME.to_string(),
        }
    }
}

impl Display for StreamName {
    /// Formats the stream name as a BigQuery resource path.
    ///
    /// Produces the fully qualified resource identifier expected by
    /// BigQuery Storage Write API in the format:
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

/// Streaming adapter that converts message batches into [`AppendRowsRequest`] objects.
///
/// Automatically chunks large batches into multiple requests while respecting
/// the 10MB BigQuery API size limit. If a single row exceeds the configured
/// limit, it is sent alone and may be rejected by the server. Implements [`Stream`] for seamless
/// integration with async streaming workflows and gRPC client operations.
#[pin_project]
#[derive(Debug)]
pub struct AppendRequestsStream<M> {
    /// Table batch containing rows and metadata for append requests.
    #[pin]
    table_batch: TableBatch<M>,
    /// Protobuf schema definition for the target table.
    proto_schema: ProtoSchema,
    /// Unique identifier for tracing and debugging requests.
    trace_id: String,
    /// Current position in the batch being processed.
    current_index: usize,
    /// Whether to include writer schema in the next request (first only).
    ///
    /// This boolean is used under the assumption that a batch of append requests belongs to the same
    /// table and has no schema differences between the rows.
    include_schema_next: bool,
    /// Shared atomic counter for tracking total bytes sent across all requests in this stream.
    bytes_sent_counter: Arc<AtomicUsize>,
}

impl<M> AppendRequestsStream<M> {
    /// Creates a new streaming adapter from a table batch.
    ///
    /// Initializes the stream with all necessary metadata for generating
    /// properly formatted append requests. The schema is included only
    /// in the first request of the stream.
    fn new(
        table_batch: TableBatch<M>,
        proto_schema: ProtoSchema,
        trace_id: String,
        bytes_sent_counter: Arc<AtomicUsize>,
    ) -> Self {
        Self {
            table_batch,
            proto_schema,
            trace_id,
            current_index: 0,
            include_schema_next: true,
            bytes_sent_counter,
        }
    }
}

impl<M> Stream for AppendRequestsStream<M>
where
    M: Message,
{
    type Item = AppendRowsRequest;

    /// Produces the next append request from the message batch.
    ///
    /// Processes messages sequentially, accumulating them into requests
    /// until the size limit is reached. Returns [`Poll::Ready(None)`]
    /// when all messages have been consumed. Each request contains the
    /// maximum number of messages that fit within size constraints.
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let rows = this.table_batch.rows();

        if *this.current_index >= rows.len() {
            return Poll::Ready(None);
        }

        let mut serialized_rows = Vec::new();
        let mut total_size = 0;
        let mut processed_count = 0;

        // Process messages from `current_index` onwards. We do not change the vector while processing
        // to avoid reallocations which are unnecessary.
        for msg in rows.iter().skip(*this.current_index) {
            // First, check the encoded length to avoid performing a full encode
            // on the first message that would exceed the limit and be dropped.
            let size = msg.encoded_len();
            if total_size + size > MAX_BATCH_SIZE_BYTES && !serialized_rows.is_empty() {
                break;
            }

            // Safe to encode now and include the row in this request chunk.
            let encoded = msg.encode_to_vec();
            debug_assert_eq!(
                encoded.len(),
                size,
                "prost::encoded_len disagrees with encode_to_vec length"
            );

            serialized_rows.push(encoded);
            total_size += size;
            processed_count += 1;
        }

        if serialized_rows.is_empty() {
            return Poll::Ready(None);
        }

        let proto_rows = ProtoRows { serialized_rows };
        let proto_data = ProtoData {
            writer_schema: if *this.include_schema_next {
                Some(this.proto_schema.clone())
            } else {
                None
            },
            rows: Some(proto_rows),
        };

        let append_rows_request = AppendRowsRequest {
            write_stream: this.table_batch.stream_name().to_string(),
            offset: None,
            trace_id: this.trace_id.clone(),
            missing_value_interpretations: HashMap::new(),
            default_missing_value_interpretation: MissingValueInterpretation::Unspecified.into(),
            rows: Some(append_rows_request::Rows::ProtoRows(proto_data)),
        };

        // Track the total bytes being sent using encoded_len
        let request_bytes = append_rows_request.encoded_len();
        this.bytes_sent_counter.fetch_add(request_bytes, Ordering::Relaxed);

        *this.current_index += processed_count;
        // After the first request, avoid sending schema again in this stream
        if *this.include_schema_next {
            *this.include_schema_next = false;
        }

        Poll::Ready(Some(append_rows_request))
    }
}

/// High-level client for BigQuery Storage Write API operations.
#[derive(Clone)]
pub struct StorageApi {
    /// Connection pool for gRPC clients to BigQuery Storage Write API.
    connection_pool: ConnectionPool,
    /// Authentication provider for API requests.
    auth: Arc<dyn Authenticator>,
    /// Base URL for BigQuery API endpoints.
    base_url: String,
    /// Semaphore for limiting concurrent gRPC requests across all operations.
    request_semaphore: Arc<Semaphore>,
}

impl StorageApi {
    /// Creates a new storage API client instance with a custom configuration.
    pub(crate) async fn with_config(auth: Arc<dyn Authenticator>, config: StorageApiConfig) -> Result<Self, BQError> {
        let connection_pool = ConnectionPool::new(config.pool_size).await?;
        let request_semaphore = Arc::new(Semaphore::new(config.max_inflight_requests));

        Ok(Self {
            connection_pool,
            auth,
            base_url: BIG_QUERY_V2_URL.to_string(),
            request_semaphore,
        })
    }

    /// Configures a custom base URL for BigQuery API endpoints.
    ///
    /// Primarily used for testing scenarios with mock or alternative
    /// BigQuery API endpoints. Returns a mutable reference for chaining.
    pub(crate) fn with_base_url(&mut self, base_url: String) -> &mut Self {
        self.base_url = base_url;
        self
    }

    /// Encodes message rows into protobuf format with size management.
    ///
    /// Processes as many rows as possible while respecting the specified
    /// size limit. Returns the encoded protobuf data and the count of
    /// rows successfully processed. When the returned count is less than
    /// the input slice length, additional calls are required for remaining rows.
    ///
    /// The size limit should be below 10MB to accommodate request metadata
    /// overhead; 9MB provides a safe margin.
    pub fn create_rows<M: Message>(
        table_descriptor: &TableDescriptor,
        rows: &[M],
        max_size_bytes: usize,
    ) -> (append_rows_request::Rows, usize) {
        let proto_schema = Self::create_proto_schema(table_descriptor);

        let mut serialized_rows = Vec::new();
        let mut total_size = 0;

        for row in rows {
            // Use encoded_len to avoid encoding a row that won't fit.
            let row_size = row.encoded_len();
            if total_size + row_size > max_size_bytes {
                break;
            }

            let encoded_row = row.encode_to_vec();
            debug_assert_eq!(
                encoded_row.len(),
                row_size,
                "prost::encoded_len disagrees with encode_to_vec length"
            );

            serialized_rows.push(encoded_row);
            total_size += row_size;
        }

        let num_rows_processed = serialized_rows.len();

        let proto_rows = ProtoRows { serialized_rows };

        let proto_data = ProtoData {
            writer_schema: Some(proto_schema),
            rows: Some(proto_rows),
        };

        (append_rows_request::Rows::ProtoRows(proto_data), num_rows_processed)
    }

    /// Retrieves metadata for a BigQuery write stream.
    ///
    /// Fetches stream information including schema definition and state
    /// details according to the specified view level. Higher view levels
    /// provide more comprehensive information but may have higher latency.
    pub async fn get_write_stream(
        &mut self,
        stream_name: &StreamName,
        view: WriteStreamView,
    ) -> Result<WriteStream, BQError> {
        let stream_name_str = stream_name.to_string();

        let get_write_stream_request = GetWriteStreamRequest {
            name: stream_name_str.clone(),
            view: view.into(),
        };

        let request = Self::new_authorized_request(self.auth.clone(), get_write_stream_request).await?;
        let mut pooled_client = self.connection_pool.get_client().await?;

        pooled_client
            .client
            .get_write_stream(request)
            .await
            .map(|resp| resp.into_inner())
            .map_err(|e| {
                warn!(stream_name = %stream_name_str, error = %e, "failed to fetch write stream metadata");
                e.into()
            })
    }

    /// Appends rows to a BigQuery table using the Storage Write API.
    ///
    /// Transmits the provided rows to the specified stream and returns
    /// a streaming response for processing results. The trace ID enables
    /// request tracking across distributed systems for debugging.
    pub async fn append_rows(
        &mut self,
        stream_name: &StreamName,
        rows: append_rows_request::Rows,
        trace_id: String,
    ) -> Result<Streaming<AppendRowsResponse>, BQError> {
        let stream_name_str = stream_name.to_string();

        let append_rows_request = AppendRowsRequest {
            write_stream: stream_name_str.clone(),
            offset: None,
            trace_id: trace_id.clone(),
            missing_value_interpretations: HashMap::new(),
            default_missing_value_interpretation: MissingValueInterpretation::Unspecified.into(),
            rows: Some(rows),
        };

        let request =
            Self::new_authorized_request(self.auth.clone(), tokio_stream::iter(vec![append_rows_request])).await?;
        let mut pooled_client = self.connection_pool.get_client().await?;

        pooled_client
            .client
            .append_rows(request)
            .await
            .map(|resp| resp.into_inner())
            .map_err(|e| {
                warn!(stream_name = %stream_name_str, trace_id = %trace_id, error = %e, "failed to append rows");
                e.into()
            })
    }

    /// Appends rows from multiple table batches with concurrent processing.
    ///
    /// Uses the shared `request_semaphore` to control parallelism globally across
    /// all StorageApi operations. Returns a collection of batch results containing
    /// responses, metadata, and bytes sent for each batch processed. Results are
    /// ordered by completion, not by submission; use `BatchAppendResult::batch_index`
    /// to correlate with the original input order.
    pub async fn append_table_batches_concurrent<M, I>(
        &self,
        table_batches: I,
        trace_id: &str,
    ) -> Result<Vec<BatchAppendResult>, BQError>
    where
        M: Message + Send + 'static,
        I: IntoIterator<Item = TableBatch<M>>,
        I::IntoIter: ExactSizeIterator,
    {
        let table_batches = table_batches.into_iter();
        let batches_num = table_batches.len();

        if batches_num == 0 {
            return Ok(Vec::new());
        }

        let semaphore = self.request_semaphore.clone();

        let mut join_set = JoinSet::new();
        for (idx, table_batch) in table_batches.enumerate() {
            // Acquire a concurrency slot and hold it until responses are fully drained.
            let permit = semaphore.clone().acquire_owned().await?;
            let trace_id = trace_id.to_string();
            let client = self.clone();

            join_set.spawn(async move {
                let stream_name = table_batch.stream_name().to_string();
                let row_count = table_batch.rows().len();

                // We compute the proto schema once for the entire batch.
                let proto_schema = Self::create_proto_schema(table_batch.table_descriptor());
                let bytes_sent_counter = Arc::new(AtomicUsize::new(0));

                // Build the request stream which will split the request into multiple requests if necessary.
                let request_stream =
                    AppendRequestsStream::new(table_batch, proto_schema, trace_id.clone(), bytes_sent_counter.clone());

                let mut batch_responses = Vec::new();

                match Self::new_authorized_request(client.auth.clone(), request_stream).await {
                    Ok(request) => match client.connection_pool.get_client().await {
                        Ok(pooled_client) => {
                            // Clone the client to get a cheap handle to the underlying HTTP/2 connection,
                            // then return the Object to the pool immediately. This allows multiple tasks
                            // to share the same connection via HTTP/2 multiplexing.
                            let mut grpc_client = pooled_client.client.clone();
                            drop(pooled_client);

                            match grpc_client.append_rows(request).await {
                                Ok(response) => {
                                    let mut streaming_response = response.into_inner();
                                    while let Some(response) = streaming_response.next().await {
                                        if let Err(e) = &response {
                                            warn!(
                                                trace_id = %trace_id,
                                                batch_index = idx,
                                                error = %e,
                                                "batch append error response"
                                            );
                                        }
                                        batch_responses.push(response);
                                    }
                                }
                                Err(status) => {
                                    warn!(
                                        trace_id = %trace_id,
                                        batch_index = idx,
                                        stream_name = %stream_name,
                                        error = %status,
                                        "failed to append batch"
                                    );
                                    batch_responses.push(Err(status));
                                }
                            }
                        }
                        Err(pool_err) => {
                            warn!(trace_id = %trace_id, batch_index = idx, error = %pool_err, "pool error");
                            batch_responses.push(Err(Status::unknown(format!("pool error: {pool_err}"))));
                        }
                    },
                    Err(err) => {
                        warn!(trace_id = %trace_id, batch_index = idx, error = %err, "auth error");
                        batch_responses.push(Err(Status::unknown(err.to_string())));
                    }
                }

                drop(permit);

                let bytes_sent = bytes_sent_counter.load(Ordering::Relaxed);
                debug!(
                    trace_id = %trace_id,
                    batch_index = idx,
                    stream_name = %stream_name,
                    row_count,
                    bytes_sent,
                    "batch completed"
                );

                BatchAppendResult::new(idx, batch_responses, bytes_sent)
            });
        }

        let mut batch_results = Vec::with_capacity(batches_num);
        while let Some(batch_result) = join_set.join_next().await {
            batch_results.push(batch_result?);
        }

        Ok(batch_results)
    }

    /// Releases all connections currently held in the connection pool.
    ///
    /// Removes all idle connections, forcing new requests to create fresh
    /// connections. Useful for recovering from network errors or refreshing
    /// stale connections.
    pub fn release_all_connections(&self) {
        self.connection_pool.release_all();
    }

    /// Creates an authenticated gRPC request with Bearer token authorization.
    ///
    /// Retrieves an access token from the authenticator and attaches it
    /// as a Bearer token in the request authorization header. Used by
    /// all Storage Write API operations requiring authentication.
    async fn new_authorized_request<T>(auth: Arc<dyn Authenticator>, message: T) -> Result<Request<T>, BQError> {
        let access_token = auth.access_token().await?;
        let bearer_token = format!("Bearer {access_token}");
        let bearer_value = bearer_token.as_str().try_into()?;

        let mut request = Request::new(message);
        let meta = request.metadata_mut();
        meta.insert("authorization", bearer_value);

        Ok(request)
    }

    /// Converts table field descriptors to protobuf field descriptors.
    ///
    /// Transforms the high-level field descriptors into the protobuf
    /// format required by BigQuery Storage Write API schema definitions.
    /// Maps column types and modes to their protobuf equivalents.
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

    /// Creates a protobuf descriptor from field descriptors.
    ///
    /// Wraps field descriptors in a [`DescriptorProto`] structure with
    /// the standard table schema name. Used as an intermediate step
    /// in protobuf schema generation.
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
    ///
    /// Creates the final [`ProtoSchema`] structure containing all
    /// field definitions required for BigQuery Storage Write API
    /// operations. This schema is included in append requests.
    fn create_proto_schema(table_descriptor: &TableDescriptor) -> ProtoSchema {
        let field_descriptors = Self::create_field_descriptors(table_descriptor);
        let proto_descriptor = Self::create_proto_descriptor(field_descriptors);

        ProtoSchema {
            proto_descriptor: Some(proto_descriptor),
        }
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
    use crate::storage::{
        ColumnMode, ColumnType, ConnectionPool, FieldDescriptor, StorageApi, StreamName, TableBatch, TableDescriptor,
    };
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
    async fn test_connection_pool() {
        let connection_pool = ConnectionPool::new(4).await.unwrap();

        // Test that we can get multiple connections from the pool
        let client1 = connection_pool.get_client().await.unwrap();
        let client2 = connection_pool.get_client().await.unwrap();

        // Connections should be different instances but both valid
        // Just verify they exist and are usable (we can't easily test the same connection
        // is reused without dropping and re-acquiring)
        assert!(std::ptr::addr_of!(*client1) != std::ptr::addr_of!(*client2));

        // Drop connections to return them to the pool
        drop(client1);
        drop(client2);

        // Get another connection to verify pool recycling works
        let client3 = connection_pool.get_client().await.unwrap();
        drop(client3);
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
    async fn test_append_table_batches_concurrent() {
        let (ref project_id, ref dataset_id, ref table_id, ref sa_key) = env_vars();
        let dataset_id = &format!("{dataset_id}_storage_table_batches");

        let mut client = Client::from_service_account_key_file(sa_key).await.unwrap();

        setup_test_table(&mut client, project_id, dataset_id, table_id)
            .await
            .unwrap();

        let table_descriptor = create_test_table_descriptor();
        let stream_name = StreamName::new_default(project_id.clone(), dataset_id.clone(), table_id.clone());
        let trace_id = "test_table_batches";

        // Create multiple table batches (all targeting the same table in this test)
        let batch1 = TableBatch::new(
            stream_name.clone(),
            table_descriptor.clone(),
            vec![
                create_test_actor(1, "John"),
                create_test_actor(2, "Jane"),
                create_test_actor(3, "Bob"),
                create_test_actor(4, "Alice"),
            ],
        );

        let batch2 = TableBatch::new(
            stream_name.clone(),
            table_descriptor.clone(),
            vec![create_test_actor(5, "Charlie"), create_test_actor(6, "Dave")],
        );

        let batch3 = TableBatch::new(stream_name, table_descriptor, vec![create_test_actor(7, "Eve")]);

        let table_batches = vec![batch1, batch2, batch3];

        // Test that all batches are processed using the default max_concurrent_requests from config.
        let batch_responses = client
            .storage_mut()
            .append_table_batches_concurrent(table_batches, trace_id)
            .await
            .unwrap();

        // We expect 3 responses per batch (one for each batch)
        assert_eq!(batch_responses.len(), 3);

        // Verify all responses are successful and track total bytes sent.
        let mut total_bytes_across_all_batches = 0;
        for batch_result in batch_responses {
            // Verify the batch was processed successfully.
            assert!(
                batch_result.is_success(),
                "Batch {} should be successful.",
                batch_result.batch_index,
            );

            // Verify each individual response for detailed error reporting.
            for response in &batch_result.responses {
                assert!(response.is_ok(), "Response should be successful: {:?}", response);
            }

            // Verify that some bytes were sent (should be greater than 0).
            let bytes_sent = batch_result.bytes_sent;
            assert!(
                bytes_sent > 0,
                "Bytes sent should be greater than 0 for batch {}, got: {}",
                batch_result.batch_index,
                bytes_sent
            );

            total_bytes_across_all_batches += bytes_sent;
        }

        // Verify that we sent bytes across all batches
        assert!(
            total_bytes_across_all_batches > 0,
            "Total bytes sent across all batches should be greater than 0"
        );
    }
}
