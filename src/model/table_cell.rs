use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    #[serde(rename = "v", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
