#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectList {
    /// The total number of projects in the list.
    pub total_items: Option<i32>,
    /// The type of list.
    pub kind: Option<String>,
    /// A hash of the page of results
    pub etag: Option<String>,
    /// A token to request the next page of results.
    pub next_page_token: Option<String>,
    // /// Projects to which you have at least READ access.
    // pub projects: Option<Vec<object>>,
}
