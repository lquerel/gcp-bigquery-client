use crate::model::table_cell::TableCell;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRow {
    /// Represents a single row in the result set, consisting of one or more fields.
    #[serde(rename = "f", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<TableCell>>,
}
