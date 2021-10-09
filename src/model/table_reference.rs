use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableReference {
    /// [Required] The ID of the dataset containing this table.
    pub dataset_id: String,
    /// [Required] The ID of the project containing this table.
    pub project_id: String,
    /// [Required] The ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.
    pub table_id: String,
}

impl TableReference {
    pub fn new(project_id: &str, dataset_id: &str, table_id: &str) -> Self {
        Self {
            dataset_id: dataset_id.into(),
            project_id: project_id.into(),
            table_id: table_id.into(),
        }
    }
}
