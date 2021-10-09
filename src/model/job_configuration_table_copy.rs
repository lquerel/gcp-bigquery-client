use crate::model::encryption_configuration::EncryptionConfiguration;
use crate::model::table_reference::TableReference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobConfigurationTableCopy {
    /// [Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_disposition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_configuration: Option<EncryptionConfiguration>,
    /// [Optional] The time when the destination table expires. Expired tables will be deleted and their storage reclaimed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_expiration_time: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_table: Option<TableReference>,
    /// [Optional] Supported operation types in table copy job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table: Option<TableReference>,
    /// [Pick one] Source tables to copy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_tables: Option<Vec<TableReference>>,
    /// [Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_disposition: Option<String>,
}
