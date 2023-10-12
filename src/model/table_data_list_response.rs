use serde::{Deserialize, Serialize};

use crate::model::table_row::TableRow;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableDataListResponse {
    /// The resource type of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Etag to the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Total rows of the entire table. In order to show default value "0", we have to present it as string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_rows: Option<String>,

    /// When this field is non-empty, it indicates that additional results are available.
    /// To request the next page of data, set the pageToken field of your next tabledata.list
    /// call to the string returned in this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,

    /// Repeated rows as result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<TableRow>>,
}
