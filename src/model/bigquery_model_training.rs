use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BigQueryModelTraining {
    /// [Output-only, Beta] Index of current ML training iteration. Updated during create model query job to show job progress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_iteration: Option<i32>,
    /// [Output-only, Beta] Expected number of iterations for the create model query job specified as num_iterations in the input query. The actual total number of iterations may be less than this number due to early stop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_total_iterations: Option<String>,
}
