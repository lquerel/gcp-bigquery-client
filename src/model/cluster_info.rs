//! Information about a single cluster for clustering model.

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterInfo {
    /// Cluster size, the total number of points assigned to the cluster.
    pub cluster_size: Option<i64>,
    /// Centroid id.
    pub centroid_id: Option<i64>,
    /// Cluster radius, the average distance from centroid to each point assigned to the cluster.
    pub cluster_radius: Option<f64>,
}
