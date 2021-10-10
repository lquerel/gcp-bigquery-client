use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutineReference {
    /// [Required] The ID of the dataset containing this routine.
    pub dataset_id: String,
    /// [Required] The ID of the project containing this routine.
    pub project_id: String,
    /// [Required] The ID of the routine. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters.
    pub routine_id: String,
}
