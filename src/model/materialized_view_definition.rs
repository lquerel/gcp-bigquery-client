use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterializedViewDefinition {
    /// [Optional] [TrustedTester] Enable automatic refresh of the materialized view when the base table is updated. The default value is \"true\".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_refresh: Option<bool>,
    /// [Output-only] [TrustedTester] The time when this materialized view was last modified, in milliseconds since the epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_time: Option<String>,
    /// [Required] A query whose result is persisted.
    pub query: String,
    /// [Optional] [TrustedTester] The maximum frequency at which this materialized view will be refreshed. The default value is \"1800000\" (30 minutes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_interval_ms: Option<String>,
}
