//! ARIMA model fitting metrics.

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArimaFittingMetrics {
    /// AIC.
    pub aic: Option<f64>,
    /// Log-likelihood.
    pub log_likelihood: Option<f64>,
    /// Variance.
    pub variance: Option<f64>,
}
