//! Global explanations containing the top most important features after training.
use crate::model::explanation::Explanation;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalExplanation {
    /// Class label for this set of global explanations. Will be empty/null for binary logistic and linear regression models. Sorted alphabetically in descending order.
    pub class_label: Option<String>,
    /// A list of the top global explanations. Sorted by absolute value of attribution in descending order.
    pub explanations: Option<Vec<Explanation>>,
}
