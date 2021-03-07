//! Information about a single cluster for clustering model.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterInfo {
    /// Centroid id.
    pub centroid_id: Option<i64>,
    /// Cluster size, the total number of points assigned to the cluster.
    pub cluster_size: Option<i64>,
    /// Cluster radius, the average distance from centroid to each point assigned to the cluster.
    pub cluster_radius: Option<f64>,
}
