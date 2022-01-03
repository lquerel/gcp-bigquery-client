use crate::model::query_parameter_type_struct_types::QueryParameterTypeStructTypes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParameterType {
    /// [Optional] The type of the array's elements, if this is an array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_type: Option<Box<QueryParameterType>>,
    /// [Optional] The types of the fields of this struct, in order, if this is a struct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub struct_types: Option<Vec<QueryParameterTypeStructTypes>>,
    /// [Required] The top level type of this field.
    #[serde(rename = "type")]
    pub r#type: String,
}
