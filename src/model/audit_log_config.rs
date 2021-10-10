use serde::{Deserialize, Serialize};

/// AuditLogConfig : Provides the configuration for logging a type of permissions. Example: { \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<LogType>,
}

/// The log type that this config enables.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogType {
    #[serde(rename = "LOG_TYPE_UNSPECIFIED")]
    LogTypeUnspecified,
    #[serde(rename = "ADMIN_READ")]
    AdminRead,
    #[serde(rename = "DATA_WRITE")]
    DataWrite,
    #[serde(rename = "DATA_READ")]
    DataRead,
}
