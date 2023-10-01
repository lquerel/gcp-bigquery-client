use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobListParameters {
    /// Whether to display jobs owned by all users in the project. Default False.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_users: Option<bool>,
    /// The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entire collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<u32>,
    /// Min value for job creation time, in milliseconds since the POSIX epoch. If set, only jobs created after or at this timestamp are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_creation_time: Option<u64>,
    /// Max value for job creation time, in milliseconds since the POSIX epoch. If set, only jobs created before or at this timestamp are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_creation_time: Option<u64>,
    /// If set, show only child jobs of the specified parent. Otherwise, show all top-level jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_job_id: Option<String>,
    /// Restrict information returned to a set of selected fields. Acceptable values are: full or minimal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
    /// Filter for job state. Acceptable values are: done, pending, and running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_filter: Option<StateFilter>,
    /// Page token, returned by a previous call, to request the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Projection {
    Full,
    Minimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StateFilter {
    Done,
    Pending,
    Running,
}
