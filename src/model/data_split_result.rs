//! Data split result. This contains references to the training and evaluation data tables that were used to train the model.
use crate::model::table_reference::TableReference;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSplitResult {
    /// Table reference of the training data after split.
    pub training_table: Option<TableReference>,
    /// Table reference of the evaluation data after split.
    pub evaluation_table: Option<TableReference>,
}
