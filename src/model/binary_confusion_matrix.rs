//! Confusion matrix for binary classification models.

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinaryConfusionMatrix {
    /// The fraction of actual positive predictions that had positive actual labels.
    pub precision: Option<f64>,
    /// Number of false samples predicted as true.
    pub false_positives: Option<i64>,
    /// Threshold value used when computing each of the following metric.
    pub positive_class_threshold: Option<f64>,
    /// The fraction of predictions given the correct label.
    pub accuracy: Option<f64>,
    /// The fraction of actual positive labels that were given a positive prediction.
    pub recall: Option<f64>,
    /// The equally weighted average of recall and precision.
    pub f_1_score: Option<f64>,
    /// Number of true samples predicted as true.
    pub true_positives: Option<i64>,
    /// Number of false samples predicted as false.
    pub false_negatives: Option<i64>,
    /// Number of true samples predicted as false.
    pub true_negatives: Option<i64>,
}
