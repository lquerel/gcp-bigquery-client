use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BqmlIterationResult {
    /// [Output-only, Beta] Time taken to run the training iteration in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<String>,
    /// [Output-only, Beta] Eval loss computed on the eval data at the end of the iteration. The eval loss is used for early stopping to avoid overfitting. No eval loss if eval_split_method option is specified as no_split or auto_split with input data size less than 500 rows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_loss: Option<f64>,
    /// [Output-only, Beta] Index of the ML training iteration, starting from zero for each training run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// [Output-only, Beta] Learning rate used for this iteration, it varies for different training iterations if learn_rate_strategy option is not constant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learn_rate: Option<f64>,
    /// [Output-only, Beta] Training loss computed on the training data at the end of the iteration. The training loss function is defined by model type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_loss: Option<f64>,
}
