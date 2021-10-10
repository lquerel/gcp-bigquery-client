use crate::model::bqml_iteration_result::BqmlIterationResult;
use crate::model::bqml_training_run_training_options::BqmlTrainingRunTrainingOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BqmlTrainingRun {
    /// [Output-only, Beta] List of each iteration results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iteration_results: Option<Vec<BqmlIterationResult>>,
    /// [Output-only, Beta] Training run start time in milliseconds since the epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// [Output-only, Beta] Different state applicable for a training run. IN PROGRESS: Training run is in progress. FAILED: Training run ended due to a non-retryable failure. SUCCEEDED: Training run successfully completed. CANCELLED: Training run cancelled by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_options: Option<BqmlTrainingRunTrainingOptions>,
}
