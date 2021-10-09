use crate::model::project_reference::ProjectReference;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectList {
    /// The total number of projects in the list.
    pub total_items: Option<i32>,
    /// The type of list.
    pub kind: Option<String>,
    /// Projects to which you have at least READ access.
    pub projects: Option<Vec<Project>>,
    /// A hash of the page of results
    pub etag: Option<String>,
    /// A token to request the next page of results.
    pub next_page_token: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// The resource type.
    pub kind: Option<String>,
    /// A unique reference to this project.
    pub project_reference: Option<ProjectReference>,
    /// The numeric ID of this project.
    pub numeric_id: Option<u64>,
    /// A descriptive name for this project.
    pub friendly_name: Option<String>,
    /// An opaque ID of this project.
    pub id: Option<String>,
}
