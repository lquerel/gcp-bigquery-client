use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Streamingbuffer {
    /// [Output-only] A lower-bound estimate of the number of bytes currently in the streaming buffer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_bytes: Option<String>,
    /// [Output-only] A lower-bound estimate of the number of rows currently in the streaming buffer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_rows: Option<String>,
    /// [Output-only] Contains the timestamp of the oldest entry in the streaming buffer, in milliseconds since the epoch, if the streaming buffer is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_entry_time: Option<String>,
}
