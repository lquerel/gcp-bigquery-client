use crate::model::data_format_options::DataFormatOptions;
use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetQueryResultsParameters {
    /// Output format adjustments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_options: Option<DataFormatOptions>,
    /// The geographic location of the job. Required except for US and EU. See details at [https://cloud.google.com/bigquery/docs/locations#specifying_your_location](https://cloud.google.com/bigquery/docs/locations#specifying_your_location).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Maximum number of results to read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// Page token, returned by a previous call, to request the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// Zero-based index of the starting row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<String>,
    /// Specifies the maximum amount of time, in milliseconds, that the client is willing to wait for the query to complete. By default, this limit is 10 seconds (10,000 milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i32>,
}
