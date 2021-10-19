//! Evaluation metrics for binary classification/classifier models.
use crate::model::aggregate_classification_metrics::AggregateClassificationMetrics;
use crate::model::binary_confusion_matrix::BinaryConfusionMatrix;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinaryClassificationMetrics {
    /// Binary confusion matrix at multiple thresholds.
    pub binary_confusion_matrix_list: Option<Vec<BinaryConfusionMatrix>>,
    /// Label representing the negative class.
    pub negative_label: Option<String>,
    /// Aggregate classification metrics.
    pub aggregate_classification_metrics: Option<AggregateClassificationMetrics>,
    /// Label representing the positive class.
    pub positive_label: Option<String>,
}
