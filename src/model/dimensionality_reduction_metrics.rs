//! Model evaluation metrics for dimensionality reduction models.

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionalityReductionMetrics {
    /// Total percentage of variance explained by the selected principal components.
    pub total_explained_variance_ratio: Option<f64>,
}
