use serde::{Deserialize, Serialize};

/// UserDefinedFunctionResource : This is used for defining User Defined Function (UDF) resources only when using legacy SQL. Users of Standard SQL should leverage either DDL (e.g. CREATE [TEMPORARY] FUNCTION ... ) or the Routines API to define UDF resources. For additional information on migrating, see: https://cloud.google.com/bigquery/docs/reference/standard-sql/migrating-from-legacy-sql#differences_in_user-defined_javascript_functions
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedFunctionResource {
    /// [Pick one] An inline resource that contains code for a user-defined function (UDF). Providing a inline code resource is equivalent to providing a URI for a file containing the same code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_code: Option<String>,
    /// [Pick one] A code resource to load from a Google Cloud Storage URI (gs://bucket/path).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
