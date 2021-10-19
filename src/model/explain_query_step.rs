use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplainQueryStep {
    /// Machine-readable operation type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Human-readable stage descriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substeps: Option<Vec<String>>,
}
