use crate::model::routine::Routine;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRoutinesResponse {
    /// A token to request the next page of results.
    pub next_page_token: Option<String>,
    /// Routines in the requested dataset. Unless read_mask is set in the request, only the following fields are populated: etag, project_id, dataset_id, routine_id, routine_type, creation_time, last_modified_time, and language.
    pub routines: Option<Vec<Routine>>,
}
