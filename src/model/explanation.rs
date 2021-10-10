//! Explanation for a single feature.

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Explanation {
    /// Full name of the feature. For non-numerical features, will be formatted like .. Overall size of feature name will always be truncated to first 120 characters.
    pub feature_name: Option<String>,
    /// Attribution of feature.
    pub attribution: Option<f64>,
}
