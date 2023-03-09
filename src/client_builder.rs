use std::path::{Path, PathBuf};

use yup_oauth2::ServiceAccountKey;

use crate::auth::{
    application_default_credentials_authenticator, authorized_user_authenticator, installed_flow_authenticator,
    service_account_authenticator, ServiceAccountAuthenticator,
};
use crate::error::BQError;
use crate::{Client, BIG_QUERY_AUTH_URL, BIG_QUERY_V2_URL, DRIVE_AUTH_URL};

pub struct ClientBuilder {
    v2_base_url: String,
    auth_base_urls: Vec<String>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            v2_base_url: BIG_QUERY_V2_URL.to_string(),
            auth_base_urls: vec![BIG_QUERY_AUTH_URL.to_string(), DRIVE_AUTH_URL.to_string()],
        }
    }

    pub fn with_v2_base_url(&mut self, base_url: String) -> &mut Self {
        self.v2_base_url = base_url;
        self
    }

    pub fn with_auth_base_url(&mut self, base_urls: Vec<String>) -> &mut Self {
        self.auth_base_urls = base_urls;
        self
    }

    pub async fn build_from_service_account_key(
        &self,
        sa_key: ServiceAccountKey,
        readonly: bool,
    ) -> Result<Client, BQError> {
        let scopes: Vec<String> = self
            .auth_base_urls
            .iter()
            .map(|auth_base_url| {
                if readonly {
                    format!("{}.readonly", auth_base_url)
                } else {
                    auth_base_url.to_string()
                }
            })
            .collect();
        let sa_auth = ServiceAccountAuthenticator::from_service_account_key(sa_key, &scopes).await?;

        let mut client = Client::from_authenticator(sa_auth);
        client.v2_base_url(self.v2_base_url.clone());
        Ok(client)
    }

    pub async fn build_from_service_account_key_file(&self, sa_key_file: &str) -> Result<Client, BQError> {
        let sa_auth = service_account_authenticator(&self.auth_base_urls, sa_key_file).await?;

        let mut client = Client::from_authenticator(sa_auth);
        client.v2_base_url(self.v2_base_url.clone());
        Ok(client)
    }

    pub async fn build_with_workload_identity(&self, readonly: bool) -> Result<Client, BQError> {
        let scope = if readonly {
            format!("{BIG_QUERY_AUTH_URL}.readonly")
        } else {
            BIG_QUERY_AUTH_URL.to_string()
        };

        let sa_auth = ServiceAccountAuthenticator::with_workload_identity(&[&scope]).await?;

        let mut client = Client::from_authenticator(sa_auth);
        client.v2_base_url(self.v2_base_url.clone());
        Ok(client)
    }

    pub async fn build_from_installed_flow_authenticator<S: AsRef<[u8]>, P: Into<PathBuf>>(
        &self,
        secret: S,
        persistant_file_path: P,
    ) -> Result<Client, BQError> {
        let auth = installed_flow_authenticator(secret, &self.auth_base_urls, persistant_file_path).await?;

        let mut client = Client::from_authenticator(auth);
        client.v2_base_url(self.v2_base_url.clone());
        Ok(client)
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
        let auth = application_default_credentials_authenticator(&self.auth_base_urls).await?;

        let mut client = Client::from_authenticator(auth);
        client.v2_base_url(self.v2_base_url.clone());
        Ok(client)
    }

    pub async fn build_from_authorized_user_authenticator<P: AsRef<Path>>(
        &self,
        authorized_user_secret_path: P,
    ) -> Result<Client, BQError> {
        let auth = authorized_user_authenticator(authorized_user_secret_path, &self.auth_base_urls).await?;

        let mut client = Client::from_authenticator(auth);
        client.v2_base_url(self.v2_base_url.clone());
        Ok(client)
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
