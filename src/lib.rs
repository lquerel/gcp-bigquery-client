use reqwest::Response;
use serde::Deserialize;

use crate::auth::service_account_authenticator;
use crate::dataset::DatasetApi;
use crate::error::BQError;
use crate::job::JobApi;
use crate::table::TableApi;
use crate::tabledata::TableDataApi;

pub mod auth;
pub mod dataset;
pub mod error;
pub mod job;
pub mod model;
pub mod table;
pub mod tabledata;

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

async fn process_response<T: for<'de> Deserialize<'de>>(resp: Response) -> Result<T, BQError> {
    if resp.status().is_success() {
        Ok(resp.json().await?)
    } else {
        Err(BQError::ResponseError {
            error: resp.json().await?,
        })
    }
}

pub struct Client {
    dataset_api: DatasetApi,
    table_api: TableApi,
    job_api: JobApi,
    tabledata_api: TableDataApi,
}

impl Client {
    pub async fn new(sa_key_file: &str) -> Self {
        let scopes = vec!["https://www.googleapis.com/auth/bigquery"];
        let sa_auth = service_account_authenticator(scopes, sa_key_file)
            .await
            .expect("expecting a valid key");

        let access_token = sa_auth.access_token().await.expect("expecting a valid token");
        let client = reqwest::Client::new();
        Self {
            dataset_api: DatasetApi::new(client.clone(), access_token.clone()),
            table_api: TableApi::new(client.clone(), access_token.clone()),
            job_api: JobApi::new(client.clone(), access_token.clone()),
            tabledata_api: TableDataApi::new(client, access_token),
        }
    }

    pub fn dataset(&self) -> &DatasetApi {
        &self.dataset_api
    }

    pub fn table(&self) -> &TableApi {
        &self.table_api
    }

    pub fn job(&self) -> &JobApi {
        &self.job_api
    }

    pub fn tabledata(&self) -> &TableDataApi {
        &self.tabledata_api
    }
}

#[cfg(test)]
pub mod tests {
    use std::env;

    pub fn env_vars() -> (String, String, String, String) {
        let project_id = env::var("PROJECT_ID").expect("PROJECT_ID env var not defined");
        let dataset_id = env::var("DATASET_ID").expect("DATASET_ID env var not defined");
        let table_id = env::var("TABLE_ID").expect("TABLE_ID env var not defined");
        let gcp_sa_key =
            env::var("GOOGLE_APPLICATION_CREDENTIALS").expect("GOOGLE_APPLICATION_CREDENTIALS env var not defined");

        println!("PROJECT_ID: {}", project_id);
        println!("DATASET_ID: {}", dataset_id);
        println!("TABLE_ID: {}", table_id);
        println!("GOOGLE_APPLICATION_CREDENTIALS: {}", gcp_sa_key);

        (project_id, dataset_id, table_id, gcp_sa_key)
    }
}
