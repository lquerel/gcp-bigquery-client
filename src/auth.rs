//! Helpers to manage GCP authentication.
use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use yup_oauth2::authenticator::Authenticator as YupAuthenticator;
use yup_oauth2::{ApplicationSecret, ServiceAccountKey};
use yup_oauth2::{InstalledFlowAuthenticator as YupInstalledFlowAuthenticator, InstalledFlowReturnMethod};

use crate::error::BQError;

#[async_trait]
pub trait Authenticator: DynClone + Send + Sync {
    async fn access_token(&self) -> Result<String, BQError>;
}

clone_trait_object!(Authenticator);

/// A service account authenticator.
#[derive(Clone)]
pub struct ServiceAccountAuthenticator {
    auth: Option<YupAuthenticator<HttpsConnector<HttpConnector>>>,
    scopes: Vec<String>,
    is_using_workload_identity: bool,
}

#[async_trait]
impl Authenticator for ServiceAccountAuthenticator {
    /// Returns an access token.
    async fn access_token(&self) -> Result<String, BQError> {
        let token = if self.is_using_workload_identity {
            get_access_token_with_workload_identity().await?.access_token
        } else {
            self.auth
                .clone()
                .unwrap()
                .token(self.scopes.as_ref())
                .await?
                .token()
                .ok_or(BQError::NoToken)?
                .to_string()
        };
        Ok(token)
    }
}

impl ServiceAccountAuthenticator {
    pub(crate) async fn from_service_account_key(
        sa_key: ServiceAccountKey,
        scopes: &[&str],
    ) -> Result<Arc<dyn Authenticator>, BQError> {
        let auth = yup_oauth2::ServiceAccountAuthenticator::builder(sa_key).build().await;

        match auth {
            Err(err) => Err(BQError::InvalidServiceAccountAuthenticator(err)),
            Ok(auth) => Ok(Arc::new(ServiceAccountAuthenticator {
                auth: Some(auth),
                scopes: scopes.iter().map(|scope| scope.to_string()).collect(),
                is_using_workload_identity: false,
            })),
        }
    }

    pub(crate) async fn with_workload_identity(scopes: &[&str]) -> Result<Arc<dyn Authenticator>, BQError> {
        Ok(Arc::new(ServiceAccountAuthenticator {
            auth: None,
            scopes: scopes.iter().map(|scope| scope.to_string()).collect(),
            is_using_workload_identity: true,
        }))
    }
}

pub(crate) async fn service_account_authenticator(
    scopes: Vec<&str>,
    sa_key_file: &str,
) -> Result<Arc<dyn Authenticator>, BQError> {
    let sa_key = yup_oauth2::read_service_account_key(sa_key_file).await?;
    ServiceAccountAuthenticator::from_service_account_key(sa_key, &scopes).await
}

#[derive(Deserialize)]
pub struct WorkloadIdentityAccessToken {
    pub access_token: String,
    pub expires_in: i32,
    pub token_type: String,
}

pub(crate) async fn get_access_token_with_workload_identity() -> Result<WorkloadIdentityAccessToken, BQError> {
    let client = reqwest::Client::new();
    let resp = client
        .get("http://metadata/computeMetadata/v1/instance/service-accounts/default/token")
        .header("Metadata-Flavor", "Google")
        .send()
        .await?;

    let content: WorkloadIdentityAccessToken = resp.json().await?;

    Ok(content)
}

#[derive(Clone)]
pub struct InstalledFlowAuthenticator {
    auth: Option<YupAuthenticator<HttpsConnector<HttpConnector>>>,
    scopes: Vec<String>,
}

impl InstalledFlowAuthenticator {
    pub(crate) async fn from_token_file_path<P: Into<PathBuf>>(
        app_secret: ApplicationSecret,
        scopes: &[&str],
        persistant_file_path: P,
    ) -> Result<Arc<dyn Authenticator>, BQError> {
        let auth = YupInstalledFlowAuthenticator::builder(app_secret, InstalledFlowReturnMethod::HTTPRedirect)
            .persist_tokens_to_disk(persistant_file_path)
            .build()
            .await;

        match auth {
            Err(err) => Err(BQError::InvalidInstalledFlowAuthenticator(err)),
            Ok(auth) => {
                // For InstalledFlowAuthenticator, we need to call token(), because it is more natural to execute the authorization code flow before returning `InstalledFlowAuthenticator` rather than before the first API call.
                match auth.token(scopes).await {
                    Err(token_err) => Err(BQError::YupAuthError(token_err)),
                    Ok(_) => Ok(Arc::new(InstalledFlowAuthenticator {
                        auth: Some(auth),
                        scopes: scopes.iter().map(|scope| scope.to_string()).collect(),
                    })),
                }
            }
        }
    }
}

#[async_trait]
impl Authenticator for InstalledFlowAuthenticator {
    async fn access_token(&self) -> Result<String, BQError> {
        Ok(self
            .auth
            .clone()
            .unwrap()
            .token(self.scopes.as_ref())
            .await?
            .token()
            .ok_or(BQError::NoToken)?
            .to_string())
    }
}

/// Send a request to Google's OAuth 2.0 server and get an access token.
/// See [Gooogle OAuth2.0 Documentation](https://developers.google.com/identity/protocols/oauth2/native-app).
pub(crate) async fn installed_flow_authenticator<S: AsRef<[u8]>, P: Into<PathBuf>>(
    secret: S,
    scopes: &[&str],
    persistant_file_path: P,
) -> Result<Arc<dyn Authenticator>, BQError> {
    let app_secret = yup_oauth2::parse_application_secret(secret)?;
    InstalledFlowAuthenticator::from_token_file_path(app_secret, scopes, persistant_file_path).await
}
