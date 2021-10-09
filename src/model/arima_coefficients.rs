//! Arima coefficients.

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArimaCoefficients {
    /// Auto-regressive coefficients, an array of double.
    pub auto_regressive_coefficients: Option<Vec<f64>>,
    /// Intercept coefficient, just a double not an array.
    pub intercept_coefficient: Option<f64>,
    /// Moving-average coefficients, an array of double.
    pub moving_average_coefficients: Option<Vec<f64>>,
}
