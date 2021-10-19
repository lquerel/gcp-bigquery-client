//! (Auto-)arima fitting result. Wrap everything in ArimaResult for easier refactoring if we want to use model-specific iteration results.
use crate::model::arima_model_info::ArimaModelInfo;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArimaResult {
    /// This message is repeated because there are multiple arima models fitted in auto-arima. For non-auto-arima model, its size is one.
    pub arima_model_info: Option<Vec<ArimaModelInfo>>,
    /// Seasonal periods. Repeated because multiple periods are supported for one time series.
    pub seasonal_periods: Option<Vec<SeasonalPeriods>>,
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
