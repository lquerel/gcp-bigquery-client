//! Manage BigQuery streaming API.
use crate::auth::ServiceAccountAuthenticator;
use crate::error::BQError;
use crate::model::table_data_insert_all_request::TableDataInsertAllRequest;
use crate::model::table_data_insert_all_response::TableDataInsertAllResponse;
use crate::{process_response, urlencode};
use reqwest::Client;

/// A table data API handler.
pub struct TableDataApi {
    client: Client,
    sa_auth: ServiceAccountAuthenticator,
}

impl TableDataApi {
    pub(crate) fn new(client: Client, sa_auth: ServiceAccountAuthenticator) -> Self {
        Self { client, sa_auth }
    }

    /// Streams data into BigQuery one record at a time without needing to run a load job. Requires the WRITER dataset
    /// role.
    pub async fn insert_all(
        &self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
        insert_request: TableDataInsertAllRequest,
    ) -> Result<TableDataInsertAllResponse, BQError> {
        let req_url = format!("https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}/insertAll", project_id=urlencode(project_id), dataset_id=urlencode(dataset_id), table_id=urlencode(table_id));

        let access_token = self.sa_auth.access_token().await?;

        let request = self
            .client
            .post(req_url.as_str())
            .bearer_auth(access_token)
            .json(&insert_request)
            .build()?;

        let resp = self.client.execute(request).await?;

        process_response(resp).await
    }
}
