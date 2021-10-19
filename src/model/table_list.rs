use crate::model::table_list_tables::TableListTables;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableList {
    /// A hash of this page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// The type of list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// A token to request the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// Tables in the requested dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<TableListTables>>,
    /// The total number of tables in the dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i32>,
}
