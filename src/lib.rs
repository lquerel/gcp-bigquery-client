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

#![allow(clippy::result_large_err)]

#[macro_use]
extern crate serde;
extern crate serde_json;

use std::env;
use std::path::PathBuf;
use std::sync::Arc;

use client_builder::ClientBuilder;
use reqwest::Response;
use serde::Deserialize;
use storage::StorageApi;
use yup_oauth2::ServiceAccountKey;

use crate::auth::Authenticator;
use crate::dataset::DatasetApi;
use crate::error::BQError;
use crate::job::JobApi;
use crate::model_api::ModelApi;
use crate::project::ProjectApi;
use crate::routine::RoutineApi;
use crate::table::TableApi;
use crate::tabledata::TableDataApi;

/// Since yup_oauth2 structs are used as parameters in public functions there is already semver
/// coupling, as it is an error if consumer uses different version of yup_oauth than gcp-bigquery-client
/// Export yup_oauth2 so consumers don't need to carefully keep their dependency versions in sync.
/// (see https://github.com/lquerel/gcp-bigquery-client/pull/86)
pub use yup_oauth2;

pub mod auth;
pub mod client_builder;
pub mod dataset;
pub mod error;
pub mod job;
pub mod model;
pub mod model_api;
pub mod project;
pub mod routine;
pub mod storage;
pub mod table;
pub mod tabledata;

const BIG_QUERY_V2_URL: &str = "https://bigquery.googleapis.com/bigquery/v2";
const BIG_QUERY_AUTH_URL: &str = "https://www.googleapis.com/auth/bigquery";

/// An asynchronous BigQuery client.
#[derive(Clone)]
pub struct Client {
    dataset_api: DatasetApi,
    table_api: TableApi,
    job_api: JobApi,
    tabledata_api: TableDataApi,
    routine_api: RoutineApi,
    model_api: ModelApi,
    project_api: ProjectApi,
    storage_api: StorageApi,
}

impl Client {
    pub async fn from_authenticator(auth: Arc<dyn Authenticator>) -> Result<Self, BQError> {
        let client = reqwest::Client::new();

        Ok(Self {
            dataset_api: DatasetApi::new(client.clone(), Arc::clone(&auth)),
            table_api: TableApi::new(client.clone(), Arc::clone(&auth)),
            job_api: JobApi::new(client.clone(), Arc::clone(&auth)),
            tabledata_api: TableDataApi::new(client.clone(), Arc::clone(&auth)),
            routine_api: RoutineApi::new(client.clone(), Arc::clone(&auth)),
            model_api: ModelApi::new(client.clone(), Arc::clone(&auth)),
            project_api: ProjectApi::new(client, Arc::clone(&auth)),
            storage_api: StorageApi::new(auth).await?,
        })
    }

    /// Constructs a new BigQuery client.
    /// # Argument
    /// * `sa_key_file` - A GCP Service Account Key file.
    pub async fn from_service_account_key_file(sa_key_file: &str) -> Result<Self, BQError> {
        ClientBuilder::new()
            .build_from_service_account_key_file(sa_key_file)
            .await
    }

    /// Constructs a new BigQuery client from a [`ServiceAccountKey`].
    /// # Argument
    /// * `sa_key` - A GCP Service Account Key `yup-oauth2` object.
    /// * `readonly` - A boolean setting whether the acquired token scope should be readonly.
    ///
    /// [`ServiceAccountKey`]: https://docs.rs/yup-oauth2/*/yup_oauth2/struct.ServiceAccountKey.html
    pub async fn from_service_account_key(sa_key: ServiceAccountKey, readonly: bool) -> Result<Self, BQError> {
        ClientBuilder::new()
            .build_from_service_account_key(sa_key, readonly)
            .await
    }

    pub async fn with_workload_identity(readonly: bool) -> Result<Self, BQError> {
        ClientBuilder::new().build_with_workload_identity(readonly).await
    }

    pub(crate) fn v2_base_url(&mut self, base_url: String) -> &mut Self {
        self.dataset_api.with_base_url(base_url.clone());
        self.table_api.with_base_url(base_url.clone());
        self.job_api.with_base_url(base_url.clone());
        self.tabledata_api.with_base_url(base_url.clone());
        self.routine_api.with_base_url(base_url.clone());
        self.model_api.with_base_url(base_url.clone());
        self.project_api.with_base_url(base_url.clone());
        self.storage_api.with_base_url(base_url);
        self
    }

    pub async fn from_installed_flow_authenticator<S: AsRef<[u8]>, P: Into<PathBuf>>(
        secret: S,
        persistant_file_path: P,
    ) -> Result<Self, BQError> {
        ClientBuilder::new()
            .build_from_installed_flow_authenticator(secret, persistant_file_path)
            .await
    }

    pub async fn from_installed_flow_authenticator_from_secret_file<P: Into<PathBuf>>(
        secret_file: &str,
        persistant_file_path: P,
    ) -> Result<Self, BQError> {
        Self::from_installed_flow_authenticator(
            tokio::fs::read(secret_file)
                .await
                .expect("expecting a valid secret file."),
            persistant_file_path,
        )
        .await
    }

    pub async fn from_application_default_credentials() -> Result<Self, BQError> {
        ClientBuilder::new().build_from_application_default_credentials().await
    }

    pub async fn from_authorized_user_secret(secret: &str) -> Result<Self, BQError> {
        ClientBuilder::new()
            .build_from_authorized_user_authenticator(secret)
            .await
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

    /// Returns a routine API handler.
    pub fn routine(&self) -> &RoutineApi {
        &self.routine_api
    }

    /// Returns a model API handler.
    pub fn model(&self) -> &ModelApi {
        &self.model_api
    }

    /// Returns a project API handler.
    pub fn project(&self) -> &ProjectApi {
        &self.project_api
    }

    /// Returns a storage API handler.
    pub fn storage(&self) -> &StorageApi {
        &self.storage_api
    }

    /// Returns a mutable storage API handler.
    pub fn storage_mut(&mut self) -> &mut StorageApi {
        &mut self.storage_api
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
    let project_id = env::var("PROJECT_ID").expect("Environment variable PROJECT_ID");
    let dataset_id = env::var("DATASET_ID").expect("Environment variable DATASET_ID");
    let table_id = env::var("TABLE_ID").expect("Environment variable TABLE_ID");
    let gcp_sa_key =
        env::var("GOOGLE_APPLICATION_CREDENTIALS").expect("Environment variable GOOGLE_APPLICATION_CREDENTIALS");

    (project_id, dataset_id, table_id, gcp_sa_key)
}

pub mod google {
    #![allow(clippy::all)]
    #[path = "google.api.rs"]
    pub mod api;

    #[path = ""]
    pub mod cloud {
        #[path = ""]
        pub mod bigquery {
            #[path = ""]
            pub mod storage {
                #![allow(clippy::all)]
                #[path = "google.cloud.bigquery.storage.v1.rs"]
                pub mod v1;
            }
        }
    }

    #[path = "google.rpc.rs"]
    pub mod rpc;
}
