use crate::model::table_data_insert_all_response_insert_errors::TableDataInsertAllResponseInsertErrors;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableDataInsertAllResponse {
    /// An array of errors for rows that were not inserted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_errors: Option<Vec<TableDataInsertAllResponseInsertErrors>>,
    /// The resource type of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
