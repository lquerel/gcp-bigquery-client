use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableFieldSchemaCategories {
    /// A list of category resource names. For example, \"projects/1/taxonomies/2/categories/3\". At most 5 categories are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
}
