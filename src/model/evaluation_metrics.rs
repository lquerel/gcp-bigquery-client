//! Evaluation metrics of a model. These are either computed on all training data or just the eval data based on whether eval data was used during training. These are not present for imported models.
use crate::model::arima_forecasting_metrics::ArimaForecastingMetrics;
use crate::model::binary_classification_metrics::BinaryClassificationMetrics;
use crate::model::clustering_metrics::ClusteringMetrics;
use crate::model::dimensionality_reduction_metrics::DimensionalityReductionMetrics;
use crate::model::multi_class_classification_metrics::MultiClassClassificationMetrics;
use crate::model::ranking_metrics::RankingMetrics;
use crate::model::regression_metrics::RegressionMetrics;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluationMetrics {
    /// Evaluation metrics when the model is a dimensionality reduction model, which currently includes PCA.
    pub dimensionality_reduction_metrics: Option<DimensionalityReductionMetrics>,
    /// Populated for implicit feedback type matrix factorization models.
    pub ranking_metrics: Option<RankingMetrics>,
    /// Populated for regression models and explicit feedback type matrix factorization models.
    pub regression_metrics: Option<RegressionMetrics>,
    /// Populated for clustering models.
    pub clustering_metrics: Option<ClusteringMetrics>,
    /// Populated for binary classification/classifier models.
    pub binary_classification_metrics: Option<BinaryClassificationMetrics>,
    /// Populated for ARIMA models.
    pub arima_forecasting_metrics: Option<ArimaForecastingMetrics>,
    /// Populated for multi-class classification/classifier models.
    pub multi_class_classification_metrics: Option<MultiClassClassificationMetrics>,
}
