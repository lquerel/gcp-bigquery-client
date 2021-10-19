//! Represents the count of a single category within the cluster.

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryCount {
    /// The name of category.
    pub category: Option<String>,
    /// The count of training samples matching the category within the cluster.
    pub count: Option<i64>,
}
