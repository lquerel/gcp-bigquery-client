//! [<img alt="github" src="https://img.shields.io/badge/github-lquerel/gcp_bigquery_client-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/lquerel/gcp-bigquery-client)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/gcp_bigquery_client.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/gcp-bigquery-client)
//! [<img alt="build status" src="https://img.shields.io/github/workflow/status/lquerel/gcp-bigquery-client/Rust/main?style=for-the-badge" height="20">](https://github.com/lquerel/gcp-bigquery-client/actions?query=branch%3Amain)
//!
//! An ergonomic async client library for GCP BigQuery.
//! * Support for dataset, table, streaming API and query (see [status section](#status) for an exhaustive list of supported API endpoints)
//! * Support Service Account Key authentication (other OAuth flows will be added later)
//! * Create tables and rows via builder patterns
//! * Persist complex Rust structs in structured BigQuery tables
//! * Async API
//!
//! <br>
//!
//! Other OAuth flows will be added later.
//!
//! For a detailed tutorial on the different ways to use GCP BigQuery Client please check out the [GitHub repository](https://github.com/lquerel/gcp-bigquery-client).
use reqwest::Response;
use serde::Deserialize;

use crate::auth::{service_account_authenticator, ServiceAccountAuthenticator};
use crate::dataset::DatasetApi;
use crate::error::BQError;
use crate::job::JobApi;
use crate::table::TableApi;
use crate::tabledata::TableDataApi;
use std::env;
use yup_oauth2::ServiceAccountKey;

pub mod auth;
pub mod dataset;
pub mod error;
pub mod job;
pub mod model;
pub mod table;
pub mod tabledata;

/// An asynchronous BigQuery client.
pub struct Client {
    dataset_api: DatasetApi,
    table_api: TableApi,
    job_api: JobApi,
    tabledata_api: TableDataApi,
}

impl Client {
    /// Constructs a new BigQuery client.
    /// # Argument
    /// * `sa_key_file` - A GCP Service Account Key file.
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

    /// Constructs a new BigQuery client from a [`ServiceAccountKey`].
    /// # Argument
    /// * `sa_key` - A GCP Service Account Key `yup-oauth2` object.
    /// * `readonly` - A boolean setting whether the acquired token scope should be readonly.
    ///
    /// [`ServiceAccountKey`]: https://docs.rs/yup-oauth2/*/yup_oauth2/struct.ServiceAccountKey.html
    pub async fn from_service_account_key(sa_key: ServiceAccountKey, readonly: bool) -> Result<Self, BQError> {
        let scopes = if readonly {
            ["https://www.googleapis.com/auth/bigquery.readonly"]
        } else {
            ["https://www.googleapis.com/auth/bigquery"]
        };
        let sa_auth = ServiceAccountAuthenticator::from_service_account_key(sa_key, &scopes).await?;

        let access_token = sa_auth.access_token().await?;
        let client = reqwest::Client::new();
        Ok(Self {
            dataset_api: DatasetApi::new(client.clone(), access_token.clone()),
            table_api: TableApi::new(client.clone(), access_token.clone()),
            job_api: JobApi::new(client.clone(), access_token.clone()),
            tabledata_api: TableDataApi::new(client, access_token),
        })
    }

    /// Returns a dataset API handler.
    pub fn dataset(&self) -> &DatasetApi {
        &self.dataset_api
    }

    /// Returns a table API handler.
    pub fn table(&self) -> &TableApi {
        &self.table_api
    }

    /// Returns a job API handler.
    pub fn job(&self) -> &JobApi {
        &self.job_api
    }

    /// Returns a table data API handler.
    pub fn tabledata(&self) -> &TableDataApi {
        &self.tabledata_api
    }
}

pub(crate) fn urlencode<T: AsRef<str>>(s: T) -> String {
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

pub fn env_vars() -> (String, String, String, String) {
    let project_id = env::var("PROJECT_ID").expect("PROJECT_ID env var not defined");
    let dataset_id = env::var("DATASET_ID").expect("DATASET_ID env var not defined");
    let table_id = env::var("TABLE_ID").expect("TABLE_ID env var not defined");
    let gcp_sa_key =
        env::var("GOOGLE_APPLICATION_CREDENTIALS").expect("GOOGLE_APPLICATION_CREDENTIALS env var not defined");

    (project_id, dataset_id, table_id, gcp_sa_key)
}
