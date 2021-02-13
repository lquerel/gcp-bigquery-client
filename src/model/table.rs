use serde::{Serialize, Deserialize};
use crate::model::clustering::Clustering;
use crate::model::encryption_configuration::EncryptionConfiguration;
use crate::model::external_data_configuration::ExternalDataConfiguration;
use crate::model::range_partitioning::RangePartitioning;
use crate::model::table_schema::TableSchema;
use crate::model::table_reference::TableReference;
use crate::model::time_partitioning::TimePartitioning;
use crate::model::view_definition::ViewDefinition;
use crate::model::materialized_view_definition::MaterializedViewDefinition;
use crate::model::model_definition::ModelDefinition;
use crate::model::snapshot_definition::SnapshotDefinition;
use crate::model::streamingbuffer::Streamingbuffer;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clustering: Option<Clustering>,
    /// [Output-only] The time when this table was created, in milliseconds since the epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// [Optional] A user-friendly description of this table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// [Output-only] A hash of the table metadata. Used to ensure there were no concurrent modifications to the resource when attempting an update. Not guaranteed to change when the table contents or the fields numRows, numBytes, numLongTermBytes or lastModifiedTime change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// [Optional] The time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed. The defaultTableExpirationMs property of the encapsulating dataset can be used to set a default expirationTime on newly created tables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_data_configuration: Option<ExternalDataConfiguration>,
    /// [Optional] A descriptive name for this table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// [Output-only] An opaque ID uniquely identifying the table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output-only] The type of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The labels associated with this table. You can use these to organize and group your tables. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// [Output-only] The time when this table was last modified, in milliseconds since the epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    /// [Output-only] The geographic location where the table resides. This value is inherited from the dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materialized_view: Option<MaterializedViewDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ModelDefinition>,
    /// [Output-only] The size of this table in bytes, excluding any data in the streaming buffer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_bytes: Option<String>,
    /// [Output-only] The number of bytes in the table that are considered \"long-term storage\".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_long_term_bytes: Option<String>,
    /// [Output-only] [TrustedTester] The physical size of this table in bytes, excluding any data in the streaming buffer. This includes compression and storage used for time travel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_physical_bytes: Option<String>,
    /// [Output-only] The number of rows of data in this table, excluding any data in the streaming buffer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_rows: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_partitioning: Option<RangePartitioning>,
    /// [Optional] If set to true, queries over this table require a partition filter that can be used for partition elimination to be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_partition_filter: Option<bool>,
    pub schema: TableSchema,
    /// [Output-only] A URL that can be used to access this resource again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_definition: Option<SnapshotDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_buffer: Option<Streamingbuffer>,
    pub table_reference: TableReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_partitioning: Option<TimePartitioning>,
    /// [Output-only] Describes the table type. The following values are supported: TABLE: A normal BigQuery table. VIEW: A virtual table defined by a SQL query. SNAPSHOT: An immutable, read-only table that is a copy of another table. [TrustedTester] MATERIALIZED_VIEW: SQL query whose result is persisted. EXTERNAL: A table that references data stored in an external storage system, such as Google Cloud Storage. The default value is TABLE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<ViewDefinition>,
}

impl Table {
    pub fn new(project_id: &str, dataset_id: &str, table_id: &str, schema: TableSchema) -> Self {
        Self {
            clustering: None,
            creation_time: None,
            description: None,
            encryption_configuration: None,
            etag: None,
            expiration_time: None,
            external_data_configuration: None,
            friendly_name: None,
            id: None,
            kind: None,
            labels: None,
            last_modified_time: None,
            location: None,
            materialized_view: None,
            model: None,
            num_bytes: None,
            num_long_term_bytes: None,
            num_physical_bytes: None,
            num_rows: None,
            range_partitioning: None,
            require_partition_filter: None,
            schema,
            self_link: None,
            snapshot_definition: None,
            streaming_buffer: None,
            table_reference: TableReference::new(project_id, dataset_id, table_id),
            time_partitioning: None,
            r#type: None,
            view: None
        }
    }
}