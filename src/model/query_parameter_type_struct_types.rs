use crate::model::query_parameter_type::QueryParameterType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParameterTypeStructTypes {
    /// [Optional] Human-oriented description of the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Optional] The name of this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<QueryParameterType>,
}
