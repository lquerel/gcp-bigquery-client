//! Evaluation metrics for regression and explicit feedback type matrix factorization models.

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegressionMetrics {
    /// Mean squared log error.
    pub mean_squared_log_error: Option<f64>,
    /// Median absolute error.
    pub median_absolute_error: Option<f64>,
    /// Mean absolute error.
    pub mean_absolute_error: Option<f64>,
    /// Mean squared error.
    pub mean_squared_error: Option<f64>,
    /// R^2 score. This corresponds to r2_score in ML.EVALUATE.
    pub r_squared: Option<f64>,
}
