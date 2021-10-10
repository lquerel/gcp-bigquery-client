use crate::model::job_configuration_extract::JobConfigurationExtract;
use crate::model::job_configuration_load::JobConfigurationLoad;
use crate::model::job_configuration_query::JobConfigurationQuery;
use crate::model::job_configuration_table_copy::JobConfigurationTableCopy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy: Option<JobConfigurationTableCopy>,
    /// [Optional] If set, don't actually run this job. A valid query will return a mostly empty response with some processing statistics, while an invalid query will return the same error it would if it wasn't a dry run. Behavior of non-query jobs is undefined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract: Option<JobConfigurationExtract>,
    /// [Optional] Job timeout in milliseconds. If this time limit is exceeded, BigQuery may attempt to terminate the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_timeout_ms: Option<String>,
    /// [Output-only] The type of the job. Can be QUERY, LOAD, EXTRACT, COPY or UNKNOWN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    /// The labels associated with this job. You can use these to organize and group your jobs. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load: Option<JobConfigurationLoad>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<JobConfigurationQuery>,
}
