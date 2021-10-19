use crate::model::explain_query_step::ExplainQueryStep;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplainQueryStage {
    /// Number of parallel input segments completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_parallel_inputs: Option<String>,
    /// Milliseconds the average shard spent on CPU-bound tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_ms_avg: Option<String>,
    /// Milliseconds the slowest shard spent on CPU-bound tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_ms_max: Option<String>,
    /// Relative amount of time the average shard spent on CPU-bound tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_ratio_avg: Option<f64>,
    /// Relative amount of time the slowest shard spent on CPU-bound tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_ratio_max: Option<f64>,
    /// Stage end time represented as milliseconds since epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_ms: Option<String>,
    /// Unique ID for stage within plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// IDs for stages that are inputs to this stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_stages: Option<Vec<String>>,
    /// Human-readable name for stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Number of parallel input segments to be processed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_inputs: Option<String>,
    /// Milliseconds the average shard spent reading input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_ms_avg: Option<String>,
    /// Milliseconds the slowest shard spent reading input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_ms_max: Option<String>,
    /// Relative amount of time the average shard spent reading input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_ratio_avg: Option<f64>,
    /// Relative amount of time the slowest shard spent reading input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_ratio_max: Option<f64>,
    /// Number of records read into the stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_read: Option<String>,
    /// Number of records written by the stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_written: Option<String>,
    /// Total number of bytes written to shuffle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shuffle_output_bytes: Option<String>,
    /// Total number of bytes written to shuffle and spilled to disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shuffle_output_bytes_spilled: Option<String>,
    /// Slot-milliseconds used by the stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_ms: Option<String>,
    /// Stage start time represented as milliseconds since epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_ms: Option<String>,
    /// Current status for the stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// List of operations within the stage in dependency order (approximately chronological).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<ExplainQueryStep>>,
    /// Milliseconds the average shard spent waiting to be scheduled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_ms_avg: Option<String>,
    /// Milliseconds the slowest shard spent waiting to be scheduled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_ms_max: Option<String>,
    /// Relative amount of time the average shard spent waiting to be scheduled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_ratio_avg: Option<f64>,
    /// Relative amount of time the slowest shard spent waiting to be scheduled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_ratio_max: Option<f64>,
    /// Milliseconds the average shard spent on writing output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_ms_avg: Option<String>,
    /// Milliseconds the slowest shard spent on writing output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_ms_max: Option<String>,
    /// Relative amount of time the average shard spent on writing output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_ratio_avg: Option<f64>,
    /// Relative amount of time the slowest shard spent on writing output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_ratio_max: Option<f64>,
}
