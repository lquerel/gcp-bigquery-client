use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInfo {
    /// [Output-only] // [Alpha] Id of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}
