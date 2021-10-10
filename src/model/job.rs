use crate::model::job_configuration::JobConfiguration;
use crate::model::job_reference::JobReference;
use crate::model::job_statistics::JobStatistics;
use crate::model::job_status::JobStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<JobConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_reference: Option<JobReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<JobStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,

    // [Output-only]
    /// [Output-only] A hash of this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// [Output-only] Opaque ID field of the job
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output-only] The type of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// [Output-only] A URL that can be used to access this resource again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// [Output-only] Email address of the user who ran the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
}
