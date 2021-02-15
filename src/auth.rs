use crate::error::BQError;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use std::fs;
use yup_oauth2::authenticator::Authenticator;

pub struct ServiceAccountAuthenticator {
    auth: Authenticator<HttpsConnector<HttpConnector>>,
    scopes: Vec<String>,
}

impl ServiceAccountAuthenticator {
    pub async fn access_token(&self) -> Result<String, BQError> {
        let token = self.auth.token(self.scopes.as_ref()).await?;
        Ok(token.as_str().to_string())
    }
}

pub async fn service_account_authenticator(
    scopes: Vec<&str>,
    sa_key_file: &str,
) -> Result<ServiceAccountAuthenticator, BQError> {
    // tmp
    let content = fs::read_to_string(sa_key_file);
    match content {
        Err(err) => println!("Error file: {}, err: {}", sa_key_file, err),
        Ok(content) => println!("content: {}", content.len()),
    }

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
