use crate::model::job::Job;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobCancelResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
    /// The resource type of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
