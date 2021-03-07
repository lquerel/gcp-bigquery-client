//! Confusion matrix for multi-class classification models.
use crate::model::row::Row;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfusionMatrix {
    /// Confidence threshold used when computing the entries of the confusion matrix.
    pub confidence_threshold: Option<f64>,
    /// One row per actual label.
    pub rows: Option<Vec<Row>>,
}
