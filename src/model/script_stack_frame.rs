use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptStackFrame {
    /// [Output-only] One-based end column.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i32>,
    /// [Output-only] One-based end line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,
    /// [Output-only] Name of the active procedure, empty if in a top-level script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure_id: Option<String>,
    /// [Output-only] One-based start column.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_column: Option<i32>,
    /// [Output-only] One-based start line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i32>,
    /// [Output-only] Text of the current statement/expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
