use crate::model::user_defined_function_resource::UserDefinedFunctionResource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewDefinition {
    /// [Required] A query that BigQuery executes when the view is referenced.
    pub query: String,
    /// Specifies whether to use BigQuery's legacy SQL for this view. The default value is true. If set to false, the view will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ Queries and views that reference this view must use the same flag value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_legacy_sql: Option<bool>,
    /// Describes user-defined function resources used in the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_function_resources: Option<Vec<UserDefinedFunctionResource>>,
}
