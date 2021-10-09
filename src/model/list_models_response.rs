use crate::model::model::Model;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListModelsResponse {
    /// A token to request the next page of results.
    pub next_page_token: Option<String>,
    /// Models in the requested dataset. Only the following fields are populated: model_reference, model_type, creation_time, last_modified_time and labels.
    pub models: Option<Vec<Model>>,
}
