//! Helpers to manage GCP authentication.
use crate::error::BQError;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use yup_oauth2::authenticator::Authenticator;

/// A service account authenticator.
pub struct ServiceAccountAuthenticator {
    auth: Authenticator<HttpsConnector<HttpConnector>>,
    scopes: Vec<String>,
}

impl ServiceAccountAuthenticator {
    /// Returns an access token.
    pub async fn access_token(&self) -> Result<String, BQError> {
        let token = self.auth.token(self.scopes.as_ref()).await?;
        Ok(token.as_str().to_string())
    }
}

pub(crate) async fn service_account_authenticator(
    scopes: Vec<&str>,
    sa_key_file: &str,
) -> Result<ServiceAccountAuthenticator, BQError> {
    let sa_key = yup_oauth2::read_service_account_key(sa_key_file).await?;
    let auth = yup_oauth2::ServiceAccountAuthenticator::builder(sa_key).build().await;

    match auth {
        Err(err) => Err(BQError::InvalidServiceAccountAuthenticator(err)),
        Ok(auth) => Ok(ServiceAccountAuthenticator {
            auth,
            scopes: scopes.iter().map(|scope| scope.to_string()).collect(),
        }),
    }
}
