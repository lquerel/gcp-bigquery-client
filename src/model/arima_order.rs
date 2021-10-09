//! Arima order, can be used for both non-seasonal and seasonal parts.

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArimaOrder {
    /// Order of the moving-average part.
    pub q: Option<i64>,
    /// Order of the autoregressive part.
    pub p: Option<i64>,
    /// Order of the differencing part.
    pub d: Option<i64>,
}
