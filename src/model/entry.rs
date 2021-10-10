//! A single entry in the confusion matrix.

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    /// The predicted label. For confidence_threshold > 0, we will also add an entry indicating the number of items under the confidence threshold.
    pub predicted_label: Option<String>,
    /// Number of items being predicted as this label.
    pub item_count: Option<i64>,
}
