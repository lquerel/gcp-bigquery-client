use crate::model::clustering::Clustering;
use crate::model::connection_property::ConnectionProperty;
use crate::model::dataset_reference::DatasetReference;
use crate::model::encryption_configuration::EncryptionConfiguration;
use crate::model::external_data_configuration::ExternalDataConfiguration;
use crate::model::query_parameter::QueryParameter;
use crate::model::range_partitioning::RangePartitioning;
use crate::model::table_reference::TableReference;
use crate::model::time_partitioning::TimePartitioning;
use crate::model::user_defined_function_resource::UserDefinedFunctionResource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobConfigurationQuery {
    /// [Optional] If true and query uses legacy SQL dialect, allows the query to produce arbitrarily large result tables at a slight cost in performance. Requires destinationTable to be set. For standard SQL queries, this flag is ignored and large results are always allowed. However, you must still set destinationTable when result size exceeds the allowed maximum response size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_large_results: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clustering: Option<Clustering>,
    /// Connection properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_properties: Option<Vec<ConnectionProperty>>,
    /// [Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_disposition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_dataset: Option<DatasetReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_table: Option<TableReference>,
    /// [Optional] If true and query uses legacy SQL dialect, flattens all nested and repeated fields in the query results. allowLargeResults must be true if this is set to false. For standard SQL queries, this flag is ignored and results are never flattened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten_results: Option<bool>,
    /// [Optional] Limits the billing tier for this job. Queries that have resource usage beyond this tier will fail (without incurring a charge). If unspecified, this will be set to your project default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_billing_tier: Option<i32>,
    /// [Optional] Limits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge). If unspecified, this will be set to your project default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bytes_billed: Option<String>,
    /// Standard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_mode: Option<String>,
    /// [Deprecated] This property is deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_nulls: Option<bool>,
    /// [Optional] Specifies a priority for the query. Possible values include INTERACTIVE and BATCH. The default value is INTERACTIVE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// [Required] SQL query text to execute. The useLegacySql field can be used to indicate whether the query uses legacy SQL or standard SQL.
    pub query: String,
    /// Query parameters for standard SQL queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<Vec<QueryParameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_partitioning: Option<RangePartitioning>,
    /// Allows the schema of the destination table to be updated as a side effect of the query job. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_update_options: Option<Vec<String>>,
    /// [Optional] If querying an external data source outside of BigQuery, describes the data format, location and other properties of the data source. By defining these properties, the data source can then be queried as if it were a standard BigQuery table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_definitions: Option<::std::collections::HashMap<String, ExternalDataConfiguration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_partitioning: Option<TimePartitioning>,
    /// Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true. If set to false, the query will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ When useLegacySql is set to false, the value of flattenResults is ignored; query will be run as if flattenResults is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_legacy_sql: Option<bool>,
    /// [Optional] Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever tables in the query are modified. Moreover, the query cache is only available when a query does not have a destination table specified. The default value is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_query_cache: Option<bool>,
    /// Describes user-defined function resources used in the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_function_resources: Option<Vec<UserDefinedFunctionResource>>,
    /// [Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_disposition: Option<String>,
}
