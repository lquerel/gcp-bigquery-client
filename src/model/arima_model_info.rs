//! Arima model information.
use crate::model::arima_coefficients::ArimaCoefficients;
use crate::model::arima_fitting_metrics::ArimaFittingMetrics;
use crate::model::arima_order::ArimaOrder;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArimaModelInfo {
    /// The time_series_id value for this time series. It will be one of the unique values from the time_series_id_column specified during ARIMA model training. Only present when time_series_id_column training option was used.
    pub time_series_id: Option<String>,
    /// Whether Arima model fitted with drift or not. It is always false when d is not 1.
    pub has_drift: Option<bool>,
    /// Seasonal periods. Repeated because multiple periods are supported for one time series.
    pub seasonal_periods: Option<Vec<SeasonalPeriods>>,
    /// Arima fitting metrics.
    pub arima_fitting_metrics: Option<ArimaFittingMetrics>,
    /// Arima coefficients.
    pub arima_coefficients: Option<ArimaCoefficients>,
    /// Non-seasonal order.
    pub non_seasonal_order: Option<ArimaOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SeasonalPeriods {
    ///
    SeasonalPeriodTypeUnspecified,
    /// No seasonality
    NoSeasonality,
    /// Daily period, 24 hours.
    Daily,
    /// Weekly period, 7 days.
    Weekly,
    /// Monthly period, 30 days or irregular.
    Monthly,
    /// Quarterly period, 90 days or irregular.
    Quarterly,
    /// Yearly period, 365 days or irregular.
    Yearly,
}
