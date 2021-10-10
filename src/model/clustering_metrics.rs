//! Evaluation metrics for clustering models.
use crate::model::cluster::Cluster;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusteringMetrics {
    /// Davies-Bouldin index.
    pub davies_bouldin_index: Option<f64>,
    /// Mean of squared distances between each sample to its cluster centroid.
    pub mean_squared_distance: Option<f64>,
    /// Information for all clusters.
    pub clusters: Option<Vec<Cluster>>,
}
