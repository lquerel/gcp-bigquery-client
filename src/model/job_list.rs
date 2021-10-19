use crate::model::job_list_jobs::JobListJobs;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobList {
    /// A hash of this page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// List of jobs that were requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<JobListJobs>>,
    /// The resource type of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// A token to request the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}
