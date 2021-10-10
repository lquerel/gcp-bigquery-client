//! Representative value of a single feature within the cluster.
use crate::model::categorical_value::CategoricalValue;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureValue {
    /// The feature column name.
    pub feature_column: Option<String>,
    /// The numerical feature value. This is the centroid value for this feature.
    pub numerical_value: Option<f64>,
    /// The categorical feature value.
    pub categorical_value: Option<CategoricalValue>,
}
