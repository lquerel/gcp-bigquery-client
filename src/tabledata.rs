//! Manage BigQuery streaming API.
use crate::auth::ServiceAccountAuthenticator;
use crate::error::BQError;
use crate::model::data_format_options::DataFormatOptions;
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
    /// # Arguments
    /// * `project_id` - Project ID of the inserted data
    /// * `dataset_id` - Dataset ID of the inserted data
    /// * `table_id` - Table ID of the inserted data
    /// * `insert_request` - Data to insert.
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

    /// Lists the content of a table in rows.
    /// # Arguments
    /// * `project_id` - Project id of the table to list.
    /// * `dataset_id` - Dataset id of the table to list.
    /// * `table_id` - Table id of the table to list.
    /// * `parameters` - Additional query parameters.
    pub async fn list(
        &self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
        parameters: ListQueryParameters,
    ) -> Result<TableDataInsertAllResponse, BQError> {
        let req_url = format!("https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}/data", project_id=urlencode(project_id), dataset_id=urlencode(dataset_id), table_id=urlencode(table_id));

        let access_token = self.sa_auth.access_token().await?;

        let request = self
            .client
            .get(req_url.as_str())
            .bearer_auth(access_token)
            .query(&parameters)
            .build()?;

        let resp = self.client.execute(request).await?;

        process_response(resp).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListQueryParameters {
    /// Start row index of the table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<String>,
    /// Row limit of the table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<u32>,
    /// Page token of the request. When this token is non-empty, it indicates additional results
    /// are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// Subset of fields to return, supports select into sub fields.
    /// Example: selectedFields = "a,e.d.f";
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_fields: Option<String>,
    /// Output timestamp field value in usec int64 instead of double. Output format adjustments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_options: Option<DataFormatOptions>,
}

#[cfg(test)]
mod test {
    use crate::error::BQError;
    use crate::model::dataset::Dataset;
    use crate::model::field_type::FieldType;
    use crate::model::table::Table;
    use crate::model::table_data_insert_all_request::TableDataInsertAllRequest;
    use crate::model::table_field_schema::TableFieldSchema;
    use crate::model::table_schema::TableSchema;
    use crate::{env_vars, Client};

    #[derive(Serialize)]
    struct Row {
        col1: String,
        col2: i64,
        col3: bool,
    }

    #[tokio::test]
    async fn test() -> Result<(), BQError> {
        let (ref project_id, ref dataset_id, ref table_id, ref sa_key) = env_vars();
        let dataset_id = &format!("{}_tabledata", dataset_id);

        let client = Client::from_service_account_key_file(sa_key).await;

        client.table().delete_if_exists(project_id, dataset_id, table_id).await;
        client.dataset().delete_if_exists(project_id, dataset_id, true).await;

        // Create dataset
        let dataset = client.dataset().create(Dataset::new(project_id, dataset_id)).await?;

        let table = dataset
            .create_table(
                &client,
                Table::from_dataset(
                    &dataset,
                    table_id,
                    TableSchema::new(vec![
                        TableFieldSchema::new("col1", FieldType::String),
                        TableFieldSchema::new("col2", FieldType::Int64),
                        TableFieldSchema::new("col3", FieldType::Boolean),
                    ]),
                ),
            )
            .await?;

        // Insert data via BigQuery Streaming API
        let mut insert_request = TableDataInsertAllRequest::new();
        insert_request.add_row(
            None,
            Row {
                col1: "val1".into(),
                col2: 2,
                col3: false,
            },
        )?;

        let result = client
            .tabledata()
            .insert_all(project_id, dataset_id, table_id, insert_request)
            .await;
        assert!(result.is_ok(), "Error: {:?}", result);

        // Remove table and dataset
        table.delete(&client).await?;
        dataset.delete(&client, true).await?;

        Ok(())
    }
}
