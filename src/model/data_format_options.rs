#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataFormatOptions {
    /// Output timestamp as usec int64. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    use_int64_timestamp: Option<bool>,
}
