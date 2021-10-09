use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParameterValue {
    /// [Optional] The array values, if this is an array type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_values: Option<Vec<QueryParameterValue>>,
    /// [Optional] The struct field values, in order of the struct type's declaration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub struct_values: Option<::std::collections::HashMap<String, QueryParameterValue>>,
    /// [Optional] The value of this value, if a simple scalar type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
