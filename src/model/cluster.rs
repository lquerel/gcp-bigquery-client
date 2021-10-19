//! Message containing the information about one cluster.
use crate::model::feature_value::FeatureValue;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    /// Values of highly variant features for this cluster.
    pub feature_values: Option<Vec<FeatureValue>>,
    /// Centroid id.
    pub centroid_id: Option<i64>,
    /// Count of training data rows that were assigned to this cluster.
    pub count: Option<i64>,
}
