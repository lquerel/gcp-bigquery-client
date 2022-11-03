use std::path::PathBuf;

use yup_oauth2::ServiceAccountKey;

use crate::auth::{service_account_authenticator, ServiceAccountAuthenticator};
use crate::error::BQError;
use crate::{installed_flow_authenticator, Client, BIG_QUERY_AUTH_URL, BIG_QUERY_V2_URL};

pub struct ClientBuilder {
    v2_base_url: String,
    auth_base_url: String,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            v2_base_url: BIG_QUERY_V2_URL.to_string(),
            auth_base_url: BIG_QUERY_AUTH_URL.to_string(),
        }
    }

    pub fn with_v2_base_url(&mut self, base_url: String) -> &mut Self {
        self.v2_base_url = base_url;
        self
    }

    pub fn with_auth_base_url(&mut self, base_url: String) -> &mut Self {
        self.auth_base_url = base_url;
        self
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

        let mut client = Client::from_authenticator(sa_auth);
        client.v2_base_url(self.v2_base_url.clone());
        Ok(client)
    }

    pub async fn build_from_service_account_key_file(&self, sa_key_file: &str) -> Client {
        let scopes = vec![self.auth_base_url.as_str()];
        let sa_auth = service_account_authenticator(scopes, sa_key_file)
            .await
            .expect("expecting a valid key");

        let mut client = Client::from_authenticator(sa_auth);
        client.v2_base_url(self.v2_base_url.clone());
        client
    }

    pub async fn build_with_workload_identity(&self, readonly: bool) -> Result<Client, BQError> {
        let scope = if readonly {
            format!("{}.readonly", BIG_QUERY_AUTH_URL)
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
        let scopes = vec![self.auth_base_url.as_str()];
        let auth = installed_flow_authenticator(secret, &scopes, persistant_file_path).await?;

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
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
