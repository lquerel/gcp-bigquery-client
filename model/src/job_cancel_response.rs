use crate::job::Job;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobCancelResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
    /// The resource type of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
