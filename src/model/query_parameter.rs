use crate::model::query_parameter_type::QueryParameterType;
use crate::model::query_parameter_value::QueryParameterValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParameter {
    /// [Optional] If unset, this is a positional parameter. Otherwise, should be unique within a query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_type: Option<QueryParameterType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<QueryParameterValue>,
}
