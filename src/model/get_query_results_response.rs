use crate::model::error_proto::ErrorProto;
use crate::model::job_reference::JobReference;
use crate::model::table_row::TableRow;
use crate::model::table_schema::TableSchema;
use serde::Deserialize;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetQueryResultsResponse {
    /// Whether the query result was fetched from the query cache.
    pub cache_hit: Option<bool>,
    /// [Output-only] The first errors or warnings encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful.
    pub errors: Option<Vec<ErrorProto>>,
    /// A hash of this response.
    pub etag: Option<String>,
    /// Whether the query has completed or not. If rows or totalRows are present, this will always be true. If this is false, totalRows will not be available.
    pub job_complete: Option<bool>,
    /// Reference to the BigQuery Job that was created to run the query. This field will be present even if the original request timed out, in which case GetQueryResults can be used to read the results once the query has completed. Since this API only returns the first page of results, subsequent pages can be fetched via the same mechanism (GetQueryResults).
    pub job_reference: Option<JobReference>,
    /// The resource type of the response.
    pub kind: Option<String>,
    /// [Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE.
    pub num_dml_affected_rows: Option<String>,
    /// A token used for paging results.
    pub page_token: Option<String>,
    /// An object with as many results as can be contained within the maximum permitted reply size. To get any additional rows, you can call GetQueryResults and specify the jobReference returned above. Present only when the query completes successfully.
    pub rows: Option<Vec<TableRow>>,
    /// The schema of the results. Present only when the query completes successfully.
    pub schema: Option<TableSchema>,
    /// The total number of bytes processed for this query.
    pub total_bytes_processed: Option<String>,
    /// The total number of rows in the complete query result set, which can be more than the number of rows in this single page of results. Present only when the query completes successfully.
    pub total_rows: Option<String>,
}
