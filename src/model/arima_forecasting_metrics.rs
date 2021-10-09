//! Model evaluation metrics for ARIMA forecasting models.
use crate::model::arima_fitting_metrics::ArimaFittingMetrics;
use crate::model::arima_order::ArimaOrder;
use crate::model::arima_single_model_forecasting_metrics::ArimaSingleModelForecastingMetrics;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArimaForecastingMetrics {
    /// Id to differentiate different time series for the large-scale case.
    pub time_series_id: Option<Vec<String>>,
    /// Seasonal periods. Repeated because multiple periods are supported for one time series.
    pub seasonal_periods: Option<Vec<SeasonalPeriods>>,
    /// Whether Arima model fitted with drift or not. It is always false when d is not 1.
    pub has_drift: Option<Vec<bool>>,
    /// Non-seasonal order.
    pub non_seasonal_order: Option<Vec<ArimaOrder>>,
    /// Repeated as there can be many metric sets (one for each model) in auto-arima and the large-scale case.
    pub arima_single_model_forecasting_metrics: Option<Vec<ArimaSingleModelForecastingMetrics>>,
    /// Arima model fitting metrics.
    pub arima_fitting_metrics: Option<Vec<ArimaFittingMetrics>>,
}

#[derive(Debug, Serialize, Deserialize)]
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
