use crate::model::clustering::Clustering;
use crate::model::range_partitioning::RangePartitioning;
use crate::model::table_list_view::TableListView;
use crate::model::table_reference::TableReference;
use crate::model::time_partitioning::TimePartitioning;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableListTables {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clustering: Option<Clustering>,
    /// The time when this table was created, in milliseconds since the epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// [Optional] The time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// The user-friendly name for this table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// An opaque ID of the table
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The resource type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The labels associated with this table. You can use these to organize and group your tables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_partitioning: Option<RangePartitioning>,
    pub table_reference: TableReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_partitioning: Option<TimePartitioning>,
    /// The type of table. Possible values are: TABLE, VIEW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<TableListView>,
}
