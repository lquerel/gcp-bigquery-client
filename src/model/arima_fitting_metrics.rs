//! ARIMA model fitting metrics.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArimaFittingMetrics {
    /// Log-likelihood.
    pub log_likelihood: Option<f64>,
    /// Variance.
    pub variance: Option<f64>,
    /// AIC.
    pub aic: Option<f64>,
}
