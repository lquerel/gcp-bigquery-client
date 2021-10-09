use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableDataInsertAllRequestRows {
    /// [Optional] A unique ID for each row. BigQuery uses this property to detect duplicate insertion requests on a best-effort basis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_id: Option<String>,
    /// Represents a single JSON object.
    // pub json: ::std::collections::HashMap<String, serde_json::Value>,
    pub json: serde_json::Value,
}
