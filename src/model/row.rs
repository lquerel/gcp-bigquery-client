//! A single row in the confusion matrix.
use crate::model::entry::Entry;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    /// The original label of this row.
    pub actual_label: Option<String>,
    /// Info describing predicted label distribution.
    pub entries: Option<Vec<Entry>>,
}
