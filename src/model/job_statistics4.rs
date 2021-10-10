use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatistics4 {
    /// [Output-only] Number of files per destination URI or URI pattern specified in the extract configuration. These values will be in the same order as the URIs specified in the 'destinationUris' field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_uri_file_counts: Option<Vec<String>>,
    /// [Output-only] Number of user bytes extracted into the result. This is the byte count as computed by BigQuery for billing purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_bytes: Option<String>,
}
