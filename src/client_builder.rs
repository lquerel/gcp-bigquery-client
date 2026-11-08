use std::path::{Path, PathBuf};
use std::sync::Arc;

use yup_oauth2::ServiceAccountKey;

use crate::auth::{
    application_default_credentials_authenticator, authorized_user_authenticator, installed_flow_authenticator,
    service_account_authenticator, Authenticator, ServiceAccountAuthenticator,
};
use crate::error::BQError;
use crate::{Client, BIG_QUERY_AUTH_URL, BIG_QUERY_V2_URL};

pub struct ClientBuilder {
    v2_base_url: String,
    auth_base_url: String,
    additional_scopes: Vec<String>,
    client: Option<reqwest::Client>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            v2_base_url: BIG_QUERY_V2_URL.to_string(),
            auth_base_url: BIG_QUERY_AUTH_URL.to_string(),
            additional_scopes: Vec::new(),
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

    /// Requests these OAuth scopes in addition to the BigQuery scope.
    ///
    /// Needed when BigQuery reads data on your behalf from another service, for example a
    /// query over an external table backed by Google Sheets or Drive, which requires
    /// [`DRIVE_READONLY_SCOPE`](crate::DRIVE_READONLY_SCOPE) (or the full Drive scope).
    /// Applies to every `build_*` method. With workload identity the scopes are sent to the
    /// metadata server as the `scopes` query parameter: Cloud Run, App Engine and Knative honor
    /// it, while plain Compute Engine ignores it and uses the instance's access scopes (the same
    /// behavior as Google's own client libraries).
    pub fn with_additional_scopes<I, S>(&mut self, scopes: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.additional_scopes = scopes.into_iter().map(Into::into).collect();
        self
    }

    /// The BigQuery scope (read-only if requested) followed by any additional scopes.
    fn scopes(&self, readonly: bool) -> Vec<String> {
        let bigquery = if readonly {
            format!("{}.readonly", self.auth_base_url)
        } else {
            self.auth_base_url.clone()
        };
        std::iter::once(bigquery)
            .chain(self.additional_scopes.iter().cloned())
            .collect()
    }

    pub async fn build_from_authenticator(&self, auth: Arc<dyn Authenticator>) -> Result<Client, BQError> {
        let mut client = Client::from_authenticator_and_client(auth, self.client.clone()).await?;
        client.v2_base_url(self.v2_base_url.clone());
        Ok(client)
    }

    pub async fn build_from_service_account_key(
        &self,
        sa_key: ServiceAccountKey,
        readonly: bool,
    ) -> Result<Client, BQError> {
        let scopes = self.scopes(readonly);
        let sa_auth = ServiceAccountAuthenticator::from_service_account_key(sa_key, &as_strs(&scopes)).await?;

        self.build_from_authenticator(sa_auth).await
    }

    pub async fn build_from_service_account_key_file(&self, sa_key_file: &str) -> Result<Client, BQError> {
        let scopes = self.scopes(false);
        let sa_auth = service_account_authenticator(as_strs(&scopes), sa_key_file).await?;

        self.build_from_authenticator(sa_auth).await
    }

    pub async fn build_with_workload_identity(&self, readonly: bool) -> Result<Client, BQError> {
        let scopes = self.scopes(readonly);
        // Only ask the metadata server for explicit scopes when extra ones were requested, so
        // the default request is unchanged (plain GCE instances use their access scopes).
        let requested = if self.additional_scopes.is_empty() {
            Vec::new()
        } else {
            as_strs(&scopes)
        };
        let sa_auth = ServiceAccountAuthenticator::with_workload_identity(&requested).await?;

        self.build_from_authenticator(sa_auth).await
    }

    pub async fn build_from_installed_flow_authenticator<S: AsRef<[u8]>, P: Into<PathBuf>>(
        &self,
        secret: S,
        persistant_file_path: P,
    ) -> Result<Client, BQError> {
        let scopes = self.scopes(false);
        let auth = installed_flow_authenticator(secret, &as_strs(&scopes), persistant_file_path).await?;

        let mut client = Client::from_authenticator_and_client(auth, self.client.clone()).await?;
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
        let scopes = self.scopes(false);
        let auth = application_default_credentials_authenticator(&as_strs(&scopes)).await?;

        let mut client = Client::from_authenticator_and_client(auth, self.client.clone()).await?;
        client.v2_base_url(self.v2_base_url.clone());
        Ok(client)
    }

    pub async fn build_from_authorized_user_authenticator<P: AsRef<Path>>(
        &self,
        authorized_user_secret_path: P,
    ) -> Result<Client, BQError> {
        let scopes = self.scopes(false);
        let auth = authorized_user_authenticator(authorized_user_secret_path, &as_strs(&scopes)).await?;

        let mut client = Client::from_authenticator_and_client(auth, self.client.clone()).await?;
        client.v2_base_url(self.v2_base_url.clone());
        Ok(client)
    }
}

fn as_strs(scopes: &[String]) -> Vec<&str> {
    scopes.iter().map(String::as_str).collect()
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DRIVE_READONLY_SCOPE;

    #[test]
    fn default_scope_is_bigquery_only() {
        let builder = ClientBuilder::new();
        assert_eq!(builder.scopes(false), vec![BIG_QUERY_AUTH_URL.to_string()]);
        assert_eq!(builder.scopes(true), vec![format!("{BIG_QUERY_AUTH_URL}.readonly")]);
    }

    #[test]
    fn additional_scopes_follow_the_bigquery_scope() {
        let mut builder = ClientBuilder::new();
        builder.with_additional_scopes([DRIVE_READONLY_SCOPE]);
        assert_eq!(
            builder.scopes(false),
            vec![BIG_QUERY_AUTH_URL.to_string(), DRIVE_READONLY_SCOPE.to_string()]
        );
        // Read-only applies to the BigQuery scope; additional scopes are passed through as given.
        assert_eq!(
            builder.scopes(true),
            vec![
                format!("{BIG_QUERY_AUTH_URL}.readonly"),
                DRIVE_READONLY_SCOPE.to_string()
            ]
        );
    }

    #[test]
    fn additional_scopes_respect_a_custom_auth_base_url() {
        let mut builder = ClientBuilder::new();
        builder
            .with_auth_base_url("https://example.test/auth/bigquery".to_string())
            .with_additional_scopes(vec!["https://example.test/auth/drive".to_string()]);
        assert_eq!(
            builder.scopes(true),
            vec![
                "https://example.test/auth/bigquery.readonly".to_string(),
                "https://example.test/auth/drive".to_string()
            ]
        );
    }

    #[test]
    fn setting_additional_scopes_replaces_previous_ones() {
        let mut builder = ClientBuilder::new();
        builder.with_additional_scopes(["a"]).with_additional_scopes(["b"]);
        assert_eq!(
            builder.scopes(false),
            vec![BIG_QUERY_AUTH_URL.to_string(), "b".to_string()]
        );
    }
}
