use crate::model::datasets::Datasets;
use crate::{urlencode, process_response};
use crate::model::dataset::Dataset;
use crate::error::BQError;
use reqwest::Client;

pub struct DatasetApi {
    client: Client,
    access_token: String,
}

impl DatasetApi {
    pub(crate) fn new(client: Client, access_token: String) -> Self {
        Self {
            client,
            access_token,
        }
    }

    /// Creates a new empty dataset.
    /// # Argument
    /// * `project-id` - Project ID of the new dataset
    pub async fn create(&self, project_id: &str, dataset: Dataset) -> Result<Dataset, BQError> {
        let req_url = &format!("https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets", project_id = urlencode(project_id));

        let request = self.client
            .post(req_url.as_str())
            .bearer_auth(&self.access_token)
            .json(&dataset)
            .build()?;

        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Lists all datasets in the specified project to which the user has been granted the READER dataset role.
    /// # Arguments
    /// * `project_id` - Project ID of the datasets to be listed
    /// * `options` - Options
    pub async fn list(&self, project_id: &str, options: ListOptions) -> Result<Datasets, BQError> {
        let req_url = &format!("https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets", project_id = urlencode(project_id));

        let mut request = self.client
            .get(req_url)
            .bearer_auth(&self.access_token);

        // process options
        if let Some(max_results) = options.max_results {
            request = request.query(&[("maxResults", max_results.to_string())]);
        }
        if let Some(page_token) = options.page_token {
            request = request.query(&[("pageToken", page_token.clone())]);
        }
        if let Some(all) = options.all {
            request = request.query(&[("all", all.to_string())]);
        }
        if let Some(filter) = options.filter {
            request = request.query(&[("filter", filter.clone())]);
        }

        let request = request.build()?;
        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Deletes the dataset specified by the datasetId value. Before you can delete a dataset, you must delete all its
    /// tables, either manually or by specifying deleteContents. Immediately after deletion, you can create another
    /// dataset with the same name.
    /// # Arguments
    /// * `project_id` - Project ID of the dataset being deleted
    /// * `dataset_id` - Dataset ID of dataset being deleted
    /// * `delete_contents` - If True, delete all the tables in the dataset. If False and the dataset contains tables, the request will fail. Default is False
    pub async fn delete(&self, project_id: &str, dataset_id: &str, delete_contents: bool) -> Result<(), BQError> {
        let req_url = &format!("https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}", project_id = urlencode(project_id), dataset_id = urlencode(dataset_id));

        let request = self.client
            .delete(req_url)
            .bearer_auth(&self.access_token)
            .query(&[("deleteContents", delete_contents.to_string())])
            .build()?;
        let response = self.client.execute(request).await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(BQError::ResponseError { error: response.json().await? })
        }
    }

    /// Returns the dataset specified by datasetID.
    /// # Arguments
    /// * `project_id` - Project ID of the requested dataset
    /// * `dataset_id` - Dataset ID of the requested dataset
    pub async fn get(&self, project_id: &str, dataset_id: &str) -> Result<Dataset, BQError> {
        let req_url = &format!("https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}", project_id = urlencode(project_id), dataset_id = urlencode(dataset_id));

        let request = self.client
            .get(req_url)
            .bearer_auth(&self.access_token)
            .build()?;
        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the
    /// patch method only replaces fields that are provided in the submitted dataset resource. This method supports
    /// patch semantics.
    /// # Arguments
    /// * dataset - The request body contains an instance of Dataset.
    pub async fn patch(&self, project_id: &str, dataset_id: &str, dataset: Dataset) -> Result<Dataset, BQError> {
        let req_url = &format!("https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}", project_id = urlencode(project_id), dataset_id = urlencode(dataset_id));

        let request = self.client
            .patch(req_url)
            .bearer_auth(&self.access_token)
            .json(&dataset)
            .build()?;
        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the
    /// patch method only replaces fields that are provided in the submitted dataset resource.
    /// # Arguments
    /// * dataset - The request body contains an instance of Dataset.
    pub async fn update(&self, project_id: &str, dataset_id: &str, dataset: Dataset) -> Result<Dataset, BQError> {
        let req_url = &format!("https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}", project_id = urlencode(project_id), dataset_id = urlencode(dataset_id));

        let request = self.client
            .put(req_url)
            .bearer_auth(&self.access_token)
            .json(&dataset)
            .build()?;
        let response = self.client.execute(request).await?;

        process_response(response).await
    }
}

pub struct ListOptions {
    max_results: Option<u64>,
    page_token: Option<String>,
    all: Option<bool>,
    filter: Option<String>,
}

impl ListOptions {
    /// The maximum number of results to return in a single response page. Leverage the page tokens to iterate through
    /// the entire collection.
    pub fn max_results(mut self, value: u64) -> Self {
        self.max_results = Some(value);
        self
    }

    /// Page token, returned by a previous call, to request the next page of results
    pub fn page_token(mut self, value: String) -> Self {
        self.page_token = Some(value);
        self
    }

    /// Whether to list all datasets, including hidden ones
    pub fn all(mut self, value: bool) -> Self {
        self.all = Some(value);
        self
    }

    /// An expression for filtering the results of the request by label. The syntax is "labels.<name>[:<value>]".
    /// Multiple filters can be ANDed together by connecting with a space. Example: "labels.department:receiving
    /// labels.active". See Filtering datasets using labels for details.
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
}

impl Default for ListOptions {
    fn default() -> Self {
        Self {
            max_results: None,
            page_token: None,
            all: None,
            filter: None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use crate::client::Client;
    use crate::dataset::{ListOptions};
    use crate::model::dataset::Dataset;
    use crate::error::BQError;
    use crate::tests::{PROJECT_ID, SA_KEY, DATASET_ID};

    #[tokio::test]
    async fn test() -> Result<(), BQError> {
        let client = Rc::new(Client::new(SA_KEY).await);

        // Create dataset
        let created_dataset = client.dataset().create(PROJECT_ID, Dataset::new(DATASET_ID)).await?;
        assert_eq!(created_dataset.id, Some(format!("{}:{}", PROJECT_ID, DATASET_ID)));

        // Get dataset
        let dataset = client.dataset().get(PROJECT_ID, DATASET_ID).await?;
        assert_eq!(dataset.id, Some(format!("{}:{}", PROJECT_ID, DATASET_ID)));

        // Patch dataset
        let dataset = client.dataset().patch(PROJECT_ID, DATASET_ID, dataset).await?;
        assert_eq!(dataset.id, Some(format!("{}:{}", PROJECT_ID, DATASET_ID)));

        // Update dataset
        let dataset = client.dataset().update(PROJECT_ID, DATASET_ID, dataset).await?;
        assert_eq!(dataset.id, Some(format!("{}:{}", PROJECT_ID, DATASET_ID)));

        // List datasets
        let datasets = client.dataset().list(PROJECT_ID, ListOptions::default().all(true)).await?;
        let mut created_dataset_found = false;
        for dataset in datasets.datasets.iter() {
            if dataset.dataset_reference.dataset_id == DATASET_ID {
                created_dataset_found = true;
            }
        }
        assert!(created_dataset_found);

        // Delete dataset
        client.dataset().delete(PROJECT_ID, DATASET_ID, true).await?;

        Ok(())
    }
}
