//! Manage BigQuery models.
use reqwest::Client;

use crate::auth::ServiceAccountAuthenticator;
use crate::error::BQError;
use crate::model::list_models_response::ListModelsResponse;
use crate::model::model::Model;
use crate::{process_response, urlencode};

/// A model API handler.
pub struct ModelApi {
    client: Client,
    sa_auth: ServiceAccountAuthenticator,
}

impl ModelApi {
    pub(crate) fn new(client: Client, sa_auth: ServiceAccountAuthenticator) -> Self {
        Self { client, sa_auth }
    }

    /// Lists all models in the specified dataset. Requires the READER dataset role.
    /// # Arguments
    /// * `project_id` - Project ID of the models to list.
    /// * `dataset_id` - Dataset ID of the models to list.
    pub async fn list(
        &self,
        project_id: &str,
        dataset_id: &str,
        options: ListOptions,
    ) -> Result<ListModelsResponse, BQError> {
        let req_url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/models",
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

    /// Deletes the model specified by modelId from the dataset.
    /// # Arguments
    /// * `project_id`- Project ID of the model to delete
    /// * `dataset_id` - Dataset ID of the model to delete
    /// * `model_id` - Model ID of the model to delete
    pub async fn delete(&self, project_id: &str, dataset_id: &str, model_id: &str) -> Result<(), BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/models/{model_id}",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
            model_id = urlencode(model_id),
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

    /// Gets the specified model resource by model ID.
    /// # Arguments
    /// * `project_id`- Project ID of the requested model
    /// * `dataset_id` - Dataset ID of the requested model
    /// * `routine_id` - Routine ID of the requested model
    pub async fn get(&self, project_id: &str, dataset_id: &str, model_id: &str) -> Result<Model, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/models/{model_id}",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
            model_id = urlencode(model_id),
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self.client.get(req_url).bearer_auth(access_token).build()?;
        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Patch specific fields in the specified model.
    /// # Arguments
    /// * `project_id`- Project ID of the model to patch
    /// * `dataset_id` - Dataset ID of the model to patch
    /// * `model_id` - Routine ID of the model to patch
    /// * `model` - Model to patch
    pub async fn update(
        &self,
        project_id: &str,
        dataset_id: &str,
        model_id: &str,
        model: Model,
    ) -> Result<Model, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/models/{model_id}",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
            model_id = urlencode(model_id),
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self
            .client
            .put(req_url)
            .bearer_auth(access_token)
            .json(&model)
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
}
