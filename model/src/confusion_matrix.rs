//! Confusion matrix for multi-class classification models.
use crate::row::Row;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfusionMatrix {
    /// One row per actual label.
    pub rows: Option<Vec<Row>>,
    /// Confidence threshold used when computing the entries of the confusion matrix.
    pub confidence_threshold: Option<f64>,
}
