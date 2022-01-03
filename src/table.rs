//! Manage BigQuery table
use log::warn;
use reqwest::Client;

use crate::auth::ServiceAccountAuthenticator;
use crate::error::BQError;
use crate::model::get_iam_policy_request::GetIamPolicyRequest;
use crate::model::policy::Policy;
use crate::model::set_iam_policy_request::SetIamPolicyRequest;
use crate::model::table::Table;
use crate::model::table_list::TableList;
use crate::model::test_iam_permissions_request::TestIamPermissionsRequest;
use crate::model::test_iam_permissions_response::TestIamPermissionsResponse;
use crate::{process_response, urlencode};

/// A table API handler.
pub struct TableApi {
    client: Client,
    sa_auth: ServiceAccountAuthenticator,
}

impl TableApi {
    pub(crate) fn new(client: Client, sa_auth: ServiceAccountAuthenticator) -> Self {
        Self { client, sa_auth }
    }

    /// Creates a new, empty table in the dataset.
    /// # Arguments
    /// * table - The request body contains an instance of Table.
    pub async fn create(&self, table: Table) -> Result<Table, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables",
            project_id = urlencode(&table.table_reference.project_id),
            dataset_id = urlencode(&table.table_reference.dataset_id)
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self
            .client
            .post(req_url.as_str())
            .bearer_auth(access_token)
            .json(&table)
            .build()?;

        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Deletes the table specified by tableId from the dataset. If the table contains data, all the data will be deleted.
    /// # Arguments
    /// * project_id - Project ID of the table to delete
    /// * dataset_id - Dataset ID of the table to delete
    /// * table_id - Table ID of the table to delete
    pub async fn delete(&self, project_id: &str, dataset_id: &str, table_id: &str) -> Result<(), BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
            table_id = urlencode(table_id)
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self.client.delete(req_url.as_str()).bearer_auth(access_token).build()?;

        let response = self.client.execute(request).await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(BQError::ResponseError {
                error: response.json().await?,
            })
        }
    }

    pub async fn delete_if_exists(&self, project_id: &str, dataset_id: &str, table_id: &str) -> bool {
        match self.delete(project_id, dataset_id, table_id).await {
            Err(BQError::ResponseError { error }) => {
                if error.error.code != 404 {
                    warn!("table.delete_if_exists: unexpected error: {:?}", error);
                }
                false
            }
            Err(err) => {
                warn!("table.delete_if_exists: unexpected error: {:?}", err);
                false
            }
            Ok(_) => true,
        }
    }

    /// Gets the specified table resource by table ID. This method does not return the data in the table, it only
    /// returns the table resource, which describes the structure of this table.
    /// # Arguments
    /// * project_id - Project ID of the table to delete
    /// * dataset_id - Dataset ID of the table to delete
    /// * table_id - Table ID of the table to delete
    /// * selected_fields - tabledata.list of table schema fields to return (comma-separated). If unspecified, all
    /// fields are returned. A fieldMask cannot be used here because the fields will automatically be converted from
    /// camelCase to snake_case and the conversion will fail if there are underscores. Since these are fields in
    /// BigQuery table schemas, underscores are allowed.
    pub async fn get(
        &self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
        selected_fields: Option<Vec<&str>>,
    ) -> Result<Table, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
            table_id = urlencode(table_id)
        );

        let access_token = self.sa_auth.access_token().await?;

        let mut request_builder = self.client.get(req_url.as_str()).bearer_auth(access_token);
        if let Some(selected_fields) = selected_fields {
            let selected_fields = selected_fields.join(",");
            request_builder = request_builder.query(&[("selectedFields", selected_fields)]);
        }

        let request = request_builder.build()?;

        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Lists all tables in the specified dataset. Requires the READER dataset role.
    /// # Arguments
    /// * project_id - Project ID of the table to delete
    /// * dataset_id - Dataset ID of the table to delete
    /// * options - Options
    pub async fn list(&self, project_id: &str, dataset_id: &str, options: ListOptions) -> Result<TableList, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id)
        );

        let access_token = self.sa_auth.access_token().await?;

        let mut request = self.client.get(req_url).bearer_auth(access_token);

        // process options
        if let Some(max_results) = options.max_results {
            request = request.query(&[("maxResults", max_results.to_string())]);
        }
        if let Some(page_token) = options.page_token {
            request = request.query(&[("pageToken", page_token)]);
        }

        let request = request.build()?;
        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Updates information in an existing table. The update method replaces the entire table resource, whereas the
    /// patch method only replaces fields that are provided in the submitted table resource. This method supports
    /// RFC5789 patch semantics.
    /// # Arguments
    /// * project_id - Project ID of the table to delete
    /// * dataset_id - Dataset ID of the table to delete
    /// * table_id - Table ID of the table to delete
    /// * table - Table to patch
    pub async fn patch(
        &self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
        table: Table,
    ) -> Result<Table, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
            table_id = urlencode(table_id)
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self
            .client
            .patch(req_url)
            .bearer_auth(access_token)
            .json(&table)
            .build()?;
        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Updates information in an existing table. The update method replaces the entire Table resource, whereas the
    /// patch method only replaces fields that are provided in the submitted Table resource.
    /// # Arguments
    /// * project_id - Project ID of the table to delete
    /// * dataset_id - Dataset ID of the table to delete
    /// * table_id - Table ID of the table to delete
    /// * table - Table to update
    pub async fn update(
        &self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
        table: Table,
    ) -> Result<Table, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
            table_id = urlencode(table_id)
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self
            .client
            .put(req_url)
            .bearer_auth(access_token)
            .json(&table)
            .build()?;
        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have
    /// a policy set.
    /// # Argument
    /// * `resource` - The resource for which the policy is being requested. See the operation documentation for the
    /// appropriate value for this field.
    pub async fn get_iam_policy(
        &self,
        resource: &str,
        get_iam_policy_request: GetIamPolicyRequest,
    ) -> Result<Policy, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{resource}/:getIamPolicy",
            resource = urlencode(resource)
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self
            .client
            .post(req_url.as_str())
            .bearer_auth(access_token)
            .json(&get_iam_policy_request)
            .build()?;

        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`,
    /// `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// # Argument
    /// * `resource` - The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field.
    pub async fn set_iam_policy(
        &self,
        resource: &str,
        set_iam_policy_request: SetIamPolicyRequest,
    ) -> Result<Policy, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{resource}/:setIamPolicy",
            resource = urlencode(resource)
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self
            .client
            .post(req_url.as_str())
            .bearer_auth(access_token)
            .json(&set_iam_policy_request)
            .build()?;

        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will
    /// return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for
    /// building permission-aware UIs and command-line tools, not for authorization checking. This operation may
    /// \"fail open\" without warning.
    /// # Argument
    /// * `resource` - The resource for which the policy detail is being requested. See the operation documentation for
    /// the appropriate value for this field.
    pub async fn test_iam_permissions(
        &self,
        resource: &str,
        test_iam_permissions_request: TestIamPermissionsRequest,
    ) -> Result<TestIamPermissionsResponse, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{resource}/:testIamPermissions",
            resource = urlencode(resource)
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self
            .client
            .post(req_url.as_str())
            .bearer_auth(access_token)
            .json(&test_iam_permissions_request)
            .build()?;

        let response = self.client.execute(request).await?;

        process_response(response).await
    }
}

/// A list of options to use with the table API handler.
#[derive(Default)]
pub struct ListOptions {
    max_results: Option<u64>,
    page_token: Option<String>,
}

impl ListOptions {
    /// The maximum number of results to return in a single response page. Leverage the page tokens to iterate
    /// through the entire collection.
    pub fn max_results(mut self, value: u64) -> Self {
        self.max_results = Some(value);
        self
    }

    /// Page token, returned by a previous call, to request the next page of results
    pub fn page_token(mut self, value: String) -> Self {
        self.page_token = Some(value);
        self
    }
}

#[cfg(test)]
mod test {
    use crate::error::BQError;
    use crate::model::dataset::Dataset;
    use crate::model::field_type::FieldType;
    use crate::model::table::Table;
    use crate::model::table_field_schema::TableFieldSchema;
    use crate::model::table_schema::TableSchema;
    use crate::table::ListOptions;
    use crate::{env_vars, Client};
    use std::time::{Duration, SystemTime};

    #[tokio::test]
    async fn test() -> Result<(), BQError> {
        let (ref project_id, ref dataset_id, ref table_id, ref sa_key) = env_vars();
        let dataset_id = &format!("{}_table", dataset_id);

        let client = Client::from_service_account_key_file(sa_key).await;

        // Delete the dataset if needed
        client.dataset().delete_if_exists(project_id, dataset_id, true).await;

        // Create dataset
        let created_dataset = client.dataset().create(Dataset::new(project_id, dataset_id)).await?;
        assert_eq!(created_dataset.id, Some(format!("{}:{}", project_id, dataset_id)));

        // Create table
        let table = Table::new(
            project_id,
            dataset_id,
            table_id,
            TableSchema::new(vec![
                TableFieldSchema::new("col1", FieldType::String),
                TableFieldSchema::new("col2", FieldType::Int64),
                TableFieldSchema::new("col3", FieldType::Boolean),
                TableFieldSchema::new("col4", FieldType::Datetime),
            ]),
        );
        let created_table = client
            .table()
            .create(
                table
                    .description("A table used for unit tests")
                    .label("owner", "me")
                    .label("env", "prod")
                    .expiration_time(SystemTime::now() + Duration::from_secs(3600)),
            )
            .await?;
        assert_eq!(created_table.table_reference.table_id, table_id.to_string());

        let table = client.table().get(project_id, dataset_id, table_id, None).await?;
        assert_eq!(table.table_reference.table_id, table_id.to_string());

        let table = client.table().update(project_id, dataset_id, table_id, table).await?;
        assert_eq!(table.table_reference.table_id, table_id.to_string());

        let table = client.table().patch(project_id, dataset_id, table_id, table).await?;
        assert_eq!(table.table_reference.table_id, table_id.to_string());

        // List tables
        let tables = client
            .table()
            .list(project_id, dataset_id, ListOptions::default())
            .await?;
        let mut created_table_found = false;
        for table_list_tables in tables.tables.unwrap().iter() {
            if &table_list_tables.table_reference.dataset_id == dataset_id {
                created_table_found = true;
            }
        }
        assert!(created_table_found);

        client.table().delete(project_id, dataset_id, table_id).await?;

        // Delete dataset
        client.dataset().delete(project_id, dataset_id, true).await?;

        Ok(())
    }
}
