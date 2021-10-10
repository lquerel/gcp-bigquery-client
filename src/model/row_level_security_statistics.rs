use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RowLevelSecurityStatistics {
    /// [Output-only] [Preview] Whether any accessed data was protected by row access policies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_security_applied: Option<bool>,
}
