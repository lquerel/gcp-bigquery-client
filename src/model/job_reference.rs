use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobReference {
    /// [Required] The ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-). The maximum length is 1,024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// The geographic location of the job. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// [Required] The ID of the project containing this job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}
