use crate::model::error_proto::ErrorProto;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableDataInsertAllResponseInsertErrors {
    /// Error information for the row indicated by the index property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorProto>>,
    /// The index of the row that error applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
}

impl Display for TableDataInsertAllResponseInsertErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}
