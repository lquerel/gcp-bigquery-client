//! Arima coefficients.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArimaCoefficients {
    /// Intercept coefficient, just a double not an array.
    pub intercept_coefficient: Option<f64>,
    /// Auto-regressive coefficients, an array of double.
    pub auto_regressive_coefficients: Option<Vec<f64>>,
    /// Moving-average coefficients, an array of double.
    pub moving_average_coefficients: Option<Vec<f64>>,
}
