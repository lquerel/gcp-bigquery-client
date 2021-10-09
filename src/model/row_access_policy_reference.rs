use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RowAccessPolicyReference {
    /// [Required] The ID of the dataset containing this row access policy.
    pub dataset_id: Option<String>,
    /// [Required] The ID of the row access policy. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters.
    pub policy_id: Option<String>,
    /// [Required] The ID of the project containing this row access policy.
    pub project_id: Option<String>,
    /// [Required] The ID of the table containing this row access policy.
    pub table_id: Option<String>,
}
