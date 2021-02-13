use crate::error::BQError;
use crate::model::table_data_insert_all_request::TableDataInsertAllRequest;
use crate::model::table_data_insert_all_response::TableDataInsertAllResponse;
use crate::{process_response, urlencode};
use reqwest::Client;

pub struct TableDataApi {
    client: Client,
    access_token: String,
}

impl TableDataApi {
    pub(crate) fn new(client: Client, access_token: String) -> Self {
        Self { client, access_token }
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

        let json = serde_json::to_string_pretty(&insert_request).unwrap();
        println!("{}", json);

        let request = self
            .client
            .post(req_url.as_str())
            .bearer_auth(&self.access_token)
            .json(&insert_request)
            .build()?;

        let resp = self.client.execute(request).await?;

        process_response(resp).await
    }
}
