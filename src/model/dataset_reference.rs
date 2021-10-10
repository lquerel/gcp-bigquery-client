use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetReference {
    /// [Required] A unique ID for this dataset, without the project name.
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_).
    /// The maximum length is 1,024 characters.
    pub dataset_id: String,
    /// The ID of the project containing this dataset.
    pub project_id: String,
}
