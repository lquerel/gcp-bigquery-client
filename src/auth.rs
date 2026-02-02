//! Helpers to manage GCP authentication.
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};
use hyper_util::client::legacy::connect::HttpConnector;
use yup_oauth2::authenticator::ApplicationDefaultCredentialsTypes;
use yup_oauth2::authenticator::Authenticator as YupAuthenticator;
use yup_oauth2::authorized_user::AuthorizedUserSecret;
use yup_oauth2::hyper_rustls::HttpsConnector;
use yup_oauth2::ApplicationDefaultCredentialsAuthenticator as YupApplicationDefaultCredentialsAuthenticator;
use yup_oauth2::ApplicationDefaultCredentialsFlowOpts;
use yup_oauth2::AuthorizedUserAuthenticator as YupAuthorizedUserAuthenticator;
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

#[derive(Clone)]
pub struct ApplicationDefaultCredentialsAuthenticator {
    auth: Option<YupAuthenticator<HttpsConnector<HttpConnector>>>,
    scopes: Vec<String>,
}

/// Helper struct to detect credential type from ADC file
#[derive(Deserialize)]
struct CredentialType {
    #[serde(rename = "type")]
    cred_type: String,
}

/// Returns the default ADC path based on the operating system.
/// - Linux/macOS: ~/.config/gcloud/application_default_credentials.json
/// - Windows: %APPDATA%\gcloud\application_default_credentials.json
fn default_adc_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA")
            .ok()
            .map(|appdata| PathBuf::from(appdata).join("gcloud/application_default_credentials.json"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOME")
            .ok()
            .map(|home| PathBuf::from(home).join(".config/gcloud/application_default_credentials.json"))
    }
}

/// Returns the ADC credential path, checking GOOGLE_APPLICATION_CREDENTIALS first,
/// then falling back to the default path.
fn adc_credential_path() -> Option<PathBuf> {
    std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
        .ok()
        .map(PathBuf::from)
        .or_else(default_adc_path)
}

/// Attempts to load authorized_user credentials from ADC path.
/// Returns None if path doesn't exist or credentials aren't authorized_user type.
async fn try_load_authorized_user_credentials(scopes: &[&str]) -> Option<Result<Arc<dyn Authenticator>, BQError>> {
    let cred_path = adc_credential_path()?;
    let contents = tokio::fs::read_to_string(&cred_path).await.ok()?;
    let cred_type: CredentialType = serde_json::from_str(&contents).ok()?;

    if cred_type.cred_type != "authorized_user" {
        return None;
    }

    let secret = match serde_json::from_str(&contents) {
        Ok(s) => s,
        Err(e) => return Some(Err(e.into())),
    };
    Some(AuthorizedUserAuthenticator::from_authorized_user_secret(secret, scopes).await)
}

impl ApplicationDefaultCredentialsAuthenticator {
    pub(crate) async fn from_scopes(scopes: &[&str]) -> Result<Arc<dyn Authenticator>, BQError> {
        // First, check if GOOGLE_APPLICATION_CREDENTIALS contains authorized_user credentials
        if let Some(result) = try_load_authorized_user_credentials(scopes).await {
            return result;
        }

        // Fall back to standard ADC flow (service_account or instance metadata)
        let opts = ApplicationDefaultCredentialsFlowOpts::default();
        let auth = match YupApplicationDefaultCredentialsAuthenticator::builder(opts).await {
            ApplicationDefaultCredentialsTypes::InstanceMetadata(auth) => auth.build().await,
            ApplicationDefaultCredentialsTypes::ServiceAccount(auth) => auth.build().await,
        };

        match auth {
            Err(err) => Err(BQError::InvalidApplicationDefaultCredentialsAuthenticator(err)),
            Ok(auth) => Ok(Arc::new(ApplicationDefaultCredentialsAuthenticator {
                auth: Some(auth),
                scopes: scopes.iter().map(|scope| scope.to_string()).collect(),
            })),
        }
    }
}

#[async_trait]
impl Authenticator for ApplicationDefaultCredentialsAuthenticator {
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

pub(crate) async fn application_default_credentials_authenticator(
    scopes: &[&str],
) -> Result<Arc<dyn Authenticator>, BQError> {
    ApplicationDefaultCredentialsAuthenticator::from_scopes(scopes).await
}

#[derive(Clone)]
pub struct AuthorizedUserAuthenticator {
    auth: Option<YupAuthenticator<HttpsConnector<HttpConnector>>>,
    scopes: Vec<String>,
}

impl AuthorizedUserAuthenticator {
    pub(crate) async fn from_authorized_user_secret(
        authorized_user_secret: AuthorizedUserSecret,
        scopes: &[&str],
    ) -> Result<Arc<dyn Authenticator>, BQError> {
        let auth = YupAuthorizedUserAuthenticator::builder(authorized_user_secret)
            .build()
            .await;

        match auth {
            Err(err) => Err(BQError::InvalidAuthorizedUserAuthenticator(err)),
            Ok(auth) => Ok(Arc::new(AuthorizedUserAuthenticator {
                auth: Some(auth),
                scopes: scopes.iter().map(|scope| scope.to_string()).collect(),
            })),
        }
    }
}

#[async_trait]
impl Authenticator for AuthorizedUserAuthenticator {
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

pub(crate) async fn authorized_user_authenticator<S: AsRef<Path>>(
    secret: S,
    scopes: &[&str],
) -> Result<Arc<dyn Authenticator>, BQError> {
    let authorized_user_secret = yup_oauth2::read_authorized_user_secret(secret).await?;
    AuthorizedUserAuthenticator::from_authorized_user_secret(authorized_user_secret, scopes).await
}

#[cfg(test)]
mod test {
    use crate::error::BQError;
    use crate::Client;
    use std::env;

    /// Test ADC authentication with authorized_user (refresh_token) credentials.
    /// Requires PROJECT_ID env var and valid ADC credentials at the default path
    /// or GOOGLE_APPLICATION_CREDENTIALS.
    #[tokio::test]
    async fn test_adc_authentication() -> Result<(), BQError> {
        let project_id = env::var("PROJECT_ID").expect("PROJECT_ID env var required");

        // Create client using ADC - works with both service_account and authorized_user
        let client = Client::from_application_default_credentials().await?;

        // Simple API call to verify authentication works
        let datasets = client.dataset().list(&project_id, Default::default()).await?;
        println!("ADC auth successful - found {} datasets", datasets.datasets.len());

        Ok(())
    }
}
