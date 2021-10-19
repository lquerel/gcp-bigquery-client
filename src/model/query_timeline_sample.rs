use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryTimelineSample {
    /// Total number of units currently being processed by workers. This does not correspond directly to slot usage. This is the largest value observed since the last sample.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_units: Option<String>,
    /// Total parallel units of work completed by this query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_units: Option<String>,
    /// Milliseconds elapsed since the start of query execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<String>,
    /// Total parallel units of work remaining for the active stages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_units: Option<String>,
    /// Cumulative slot-ms consumed by the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_slot_ms: Option<String>,
}
