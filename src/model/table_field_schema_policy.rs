use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableFieldSchemaPolicyTags {
    /// A list of category resource names. For example, \"projects/1/location/eu/taxonomies/2/policyTags/3\". At most 1 policy tag is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
}
