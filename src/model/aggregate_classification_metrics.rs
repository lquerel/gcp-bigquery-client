//! Aggregate metrics for classification/classifier models. For multi-class models, the metrics are either macro-averaged or micro-averaged. When macro-averaged, the metrics are calculated for each label and then an unweighted average is taken of those values. When micro-averaged, the metric is calculated globally by counting the total number of correctly predicted rows.

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateClassificationMetrics {
    /// Area Under a ROC Curve. For multiclass this is a macro-averaged metric.
    pub roc_auc: Option<f64>,
    /// Precision is the fraction of actual positive predictions that had positive actual labels. For multiclass this is a macro-averaged metric treating each class as a binary classifier.
    pub precision: Option<f64>,
    /// The F1 score is an average of recall and precision. For multiclass this is a macro-averaged metric.
    pub f_1_score: Option<f64>,
    /// Accuracy is the fraction of predictions given the correct label. For multiclass this is a micro-averaged metric.
    pub accuracy: Option<f64>,
    /// Threshold at which the metrics are computed. For binary classification models this is the positive class threshold. For multi-class classfication models this is the confidence threshold.
    pub threshold: Option<f64>,
    /// Logarithmic Loss. For multiclass this is a macro-averaged metric.
    pub log_loss: Option<f64>,
    /// Recall is the fraction of actual positive labels that were given a positive prediction. For multiclass this is a macro-averaged metric.
    pub recall: Option<f64>,
}
