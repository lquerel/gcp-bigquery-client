//! Manage BigQuery user-defined function or a stored procedure.
use reqwest::Client;

use crate::auth::ServiceAccountAuthenticator;
use crate::error::BQError;
use crate::model::list_routines_response::ListRoutinesResponse;
use crate::model::routine::Routine;
use crate::{process_response, urlencode};

/// A routine API handler.
pub struct RoutineApi {
    client: Client,
    sa_auth: ServiceAccountAuthenticator,
}

impl RoutineApi {
    pub(crate) fn new(client: Client, sa_auth: ServiceAccountAuthenticator) -> Self {
        Self { client, sa_auth }
    }

    /// Creates a new routine in the dataset.
    /// # Arguments
    /// * `project_id` - Project ID of the new routine.
    /// * `dataset_id` - Dataset ID of the new routine.
    /// * `routine` - The request body contains an instance of Routine.
    pub async fn insert(&self, project_id: &str, dataset_id: &str, routine: Routine) -> Result<Routine, BQError> {
        let req_url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/routines",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self
            .client
            .post(req_url.as_str())
            .bearer_auth(access_token)
            .json(&routine)
            .build()?;

        let resp = self.client.execute(request).await?;

        process_response(resp).await
    }

    /// Lists all routines in the specified dataset. Requires the READER dataset role.
    /// # Arguments
    /// * `project_id` - Project ID of the routines to list.
    /// * `dataset_id` - Dataset ID of the routines to list.
    pub async fn list(
        &self,
        project_id: &str,
        dataset_id: &str,
        options: ListOptions,
    ) -> Result<ListRoutinesResponse, BQError> {
        let req_url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/routines",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self
            .client
            .get(req_url.as_str())
            .bearer_auth(access_token)
            .query(&options)
            .build()?;

        let resp = self.client.execute(request).await?;

        process_response(resp).await
    }

    /// Deletes the routine specified by routineId from the dataset.
    /// # Arguments
    /// * `project_id`- Project ID of the routine to delete
    /// * `dataset_id` - Dataset ID of the routine to delete
    /// * `routine_id` - Routine ID of the routine to delete
    pub async fn delete(&self, project_id: &str, dataset_id: &str, routine_id: &str) -> Result<(), BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/routines/{routine_id}",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
            routine_id = urlencode(routine_id),
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self.client.delete(req_url).bearer_auth(access_token).build()?;
        let response = self.client.execute(request).await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(BQError::ResponseError {
                error: response.json().await?,
            })
        }
    }

    /// Gets the specified routine resource by routine ID.
    /// # Arguments
    /// * `project_id`- Project ID of the requested routine
    /// * `dataset_id` - Dataset ID of the requested routine
    /// * `routine_id` - Routine ID of the requested routine
    pub async fn get(&self, project_id: &str, dataset_id: &str, routine_id: &str) -> Result<Routine, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/routines/{routine_id}",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
            routine_id = urlencode(routine_id),
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self.client.get(req_url).bearer_auth(access_token).build()?;
        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Updates information in an existing routine. The update method replaces the entire Routine
    /// resource.
    /// # Arguments
    /// * `project_id`- Project ID of the routine to update
    /// * `dataset_id` - Dataset ID of the routine to update
    /// * `routine_id` - Routine ID of the routine to update
    /// * `routine` - Routine to update
    pub async fn update(
        &self,
        project_id: &str,
        dataset_id: &str,
        routine_id: &str,
        routine: Routine,
    ) -> Result<Routine, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/routines/{routine_id}",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
            routine_id = urlencode(routine_id),
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self
            .client
            .put(req_url)
            .bearer_auth(access_token)
            .json(&routine)
            .build()?;
        let response = self.client.execute(request).await?;

        process_response(response).await
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListOptions {
    max_results: Option<u64>,
    page_token: Option<String>,
    read_mask: Option<String>,
    filter: Option<String>,
}

impl ListOptions {
    /// The maximum number of results to return in a single response page. Leverage the page tokens
    /// to iterate through the entire collection.
    pub fn max_results(mut self, value: u64) -> Self {
        self.max_results = Some(value);
        self
    }

    /// Page token, returned by a previous call, to request the next page of results
    pub fn page_token(mut self, value: String) -> Self {
        self.page_token = Some(value);
        self
    }

    /// If set, then only the Routine fields in the field mask, as well as projectId, datasetId and
    /// routineId, are returned in the response. If unset, then the following Routine fields are
    /// returned: etag, projectId, datasetId, routineId, routineType, creationTime,
    /// lastModifiedTime, and language.
    ///
    /// A comma-separated list of fully qualified names of fields. Example: "user.displayName,photo".
    pub fn read_mask(mut self, value: String) -> Self {
        self.read_mask = Some(value);
        self
    }

    /// If set, then only the Routines matching this filter are returned. The current supported form
    /// is either "routineType:" or "routineType:", where is a RoutineType enum.
    /// Example: "routineType:SCALAR_FUNCTION".
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
}
