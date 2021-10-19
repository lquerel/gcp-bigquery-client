//! Representative value of a categorical feature.
use crate::model::category_count::CategoryCount;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoricalValue {
    /// Counts of all categories for the categorical feature. If there are more than ten categories, we return top ten (by count) and return one more CategoryCount with category "_OTHER_" and count as aggregate counts of remaining categories.
    pub category_counts: Option<Vec<CategoryCount>>,
}
