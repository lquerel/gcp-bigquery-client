use crate::model::error_proto::ErrorProto;
use crate::model::job_configuration::JobConfiguration;
use crate::model::job_reference::JobReference;
use crate::model::job_statistics::JobStatistics;
use crate::model::job_status::JobStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobListJobs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<JobConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_result: Option<ErrorProto>,
    /// Unique opaque ID of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_reference: Option<JobReference>,
    /// The resource type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Running state of the job. When the state is DONE, errorResult can be checked to determine whether the job succeeded or failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<JobStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
    /// [Full-projection-only] Email address of the user who ran the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
}
