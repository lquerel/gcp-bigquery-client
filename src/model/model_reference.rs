use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelReference {
    /// [Required] The ID of the dataset containing this model.
    pub dataset_id: String,
    /// [Required] The ID of the model. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.
    pub model_id: String,
    /// [Required] The ID of the project containing this model.
    pub project_id: String,
}
