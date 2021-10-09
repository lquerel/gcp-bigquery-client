use serde::{Deserialize, Serialize};

/// BqmlTrainingRunTrainingOptions : [Output-only, Beta] Training options used by this training run. These options are mutable for subsequent training runs. Default values are explicitly stored for options not specified in the input query of the first training run. For subsequent training runs, any option not explicitly specified in the input query will be copied from the previous training run.

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BqmlTrainingRunTrainingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_stop: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l1_reg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l2_reg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learn_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learn_rate_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_search_init_learn_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iteration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_rel_progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_start: Option<bool>,
}
