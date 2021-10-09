use crate::model::script_stack_frame::ScriptStackFrame;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptStatistics {
    /// [Output-only] Whether this child job was a statement or expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_kind: Option<String>,
    /// Stack trace showing the line/column/procedure name of each frame on the stack at the point where the current evaluation happened. The leaf frame is first, the primary script is last. Never empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_frames: Option<Vec<ScriptStackFrame>>,
}
