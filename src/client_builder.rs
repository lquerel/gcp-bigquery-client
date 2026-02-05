use std::path::{Path, PathBuf};
use std::sync::Arc;

use yup_oauth2::ServiceAccountKey;

use crate::auth::{
    application_default_credentials_authenticator, authorized_user_authenticator, installed_flow_authenticator,
    service_account_authenticator, Authenticator, ServiceAccountAuthenticator,
};
use crate::dataset::DatasetApi;
use crate::error::BQError;
use crate::job::JobApi;
use crate::model_api::ModelApi;
use crate::project::ProjectApi;
use crate::routine::RoutineApi;
use crate::storage::{StorageApi, StorageApiConfig};
use crate::table::TableApi;
use crate::tabledata::TableDataApi;
use crate::{Client, BIG_QUERY_AUTH_URL, BIG_QUERY_V2_URL};

pub struct ClientBuilder {
    v2_base_url: String,
    auth_base_url: String,
    storage_config: StorageApiConfig,
    client: Option<reqwest::Client>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            v2_base_url: BIG_QUERY_V2_URL.to_string(),
            auth_base_url: BIG_QUERY_AUTH_URL.to_string(),
            storage_config: StorageApiConfig::default(),
            client: None,
        }
    }

    pub fn with_v2_base_url(&mut self, base_url: String) -> &mut Self {
        self.v2_base_url = base_url;
        self
    }

    pub fn with_client(&mut self, client: reqwest::Client) -> &mut Self {
        self.client = Some(client);
        self
    }

    pub fn with_auth_base_url(&mut self, base_url: String) -> &mut Self {
        self.auth_base_url = base_url;
        self
    }

    /// Sets the configuration for the BigQuery Storage Write API.
    pub fn with_storage_config(&mut self, config: StorageApiConfig) -> &mut Self {
        self.storage_config = config;
        self
    }

    pub async fn build_from_authenticator(&self, auth: Arc<dyn Authenticator>) -> Result<Client, BQError> {
        let http_client = self.client.clone().unwrap_or_default();

        let mut dataset_api = DatasetApi::new(http_client.clone(), Arc::clone(&auth));
        let mut table_api = TableApi::new(http_client.clone(), Arc::clone(&auth));
        let mut job_api = JobApi::new(http_client.clone(), Arc::clone(&auth));
        let mut tabledata_api = TableDataApi::new(http_client.clone(), Arc::clone(&auth));
        let mut routine_api = RoutineApi::new(http_client.clone(), Arc::clone(&auth));
        let mut model_api = ModelApi::new(http_client.clone(), Arc::clone(&auth));
        let mut project_api = ProjectApi::new(http_client, Arc::clone(&auth));
        let mut storage_api = StorageApi::with_config(auth, self.storage_config.clone()).await?;

        dataset_api.with_base_url(self.v2_base_url.clone());
        table_api.with_base_url(self.v2_base_url.clone());
        job_api.with_base_url(self.v2_base_url.clone());
        tabledata_api.with_base_url(self.v2_base_url.clone());
        routine_api.with_base_url(self.v2_base_url.clone());
        model_api.with_base_url(self.v2_base_url.clone());
        project_api.with_base_url(self.v2_base_url.clone());
        storage_api.with_base_url(self.v2_base_url.clone());

        Ok(Client::new(
            dataset_api,
            table_api,
            job_api,
            tabledata_api,
            routine_api,
            model_api,
            project_api,
            storage_api,
        ))
    }

    pub async fn build_from_service_account_key(
        &self,
        sa_key: ServiceAccountKey,
        readonly: bool,
    ) -> Result<Client, BQError> {
        let scope = if readonly {
            format!("{}.readonly", self.auth_base_url)
        } else {
            self.auth_base_url.clone()
        };
        let sa_auth = ServiceAccountAuthenticator::from_service_account_key(sa_key, &[&scope]).await?;

        self.build_from_authenticator(sa_auth).await
    }

    pub async fn build_from_service_account_key_file(&self, sa_key_file: &str) -> Result<Client, BQError> {
        let scopes = vec![self.auth_base_url.as_str()];
        let sa_auth = service_account_authenticator(scopes, sa_key_file).await?;

        self.build_from_authenticator(sa_auth).await
    }

    pub async fn build_with_workload_identity(&self, readonly: bool) -> Result<Client, BQError> {
        let scope = if readonly {
            format!("{BIG_QUERY_AUTH_URL}.readonly")
        } else {
            BIG_QUERY_AUTH_URL.to_string()
        };

        let sa_auth = ServiceAccountAuthenticator::with_workload_identity(&[&scope]).await?;

        self.build_from_authenticator(sa_auth).await
    }

    pub async fn build_from_installed_flow_authenticator<S: AsRef<[u8]>, P: Into<PathBuf>>(
        &self,
        secret: S,
        persistant_file_path: P,
    ) -> Result<Client, BQError> {
        let scopes = vec![self.auth_base_url.as_str()];
        let auth = installed_flow_authenticator(secret, &scopes, persistant_file_path).await?;

        self.build_from_authenticator(auth).await
    }

    pub async fn build_from_installed_flow_authenticator_from_secret_file<P: Into<PathBuf>>(
        &self,
        secret_file: &str,
        persistant_file_path: P,
    ) -> Result<Client, BQError> {
        self.build_from_installed_flow_authenticator(
            tokio::fs::read(secret_file)
                .await
                .expect("expecting a valid secret file."),
            persistant_file_path,
        )
        .await
    }

    pub async fn build_from_application_default_credentials(&self) -> Result<Client, BQError> {
        let scopes = vec![self.auth_base_url.as_str()];
        let auth = application_default_credentials_authenticator(&scopes).await?;

        self.build_from_authenticator(auth).await
    }

    pub async fn build_from_authorized_user_authenticator<P: AsRef<Path>>(
        &self,
        authorized_user_secret_path: P,
    ) -> Result<Client, BQError> {
        let scopes = vec![self.auth_base_url.as_str()];
        let auth = authorized_user_authenticator(authorized_user_secret_path, &scopes).await?;

        self.build_from_authenticator(auth).await
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
