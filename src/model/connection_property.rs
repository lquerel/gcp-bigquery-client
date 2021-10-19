use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionProperty {
    /// [Required] Name of the connection property to set.
    pub key: String,
    /// [Required] Value of the connection property.
    pub value: String,
}
