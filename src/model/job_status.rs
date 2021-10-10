use crate::model::error_proto::ErrorProto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_result: Option<ErrorProto>,
    /// [Output-only] The first errors encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorProto>>,
    /// [Output-only] Running state of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
