//! Workload identity talks to the metadata server; check which scopes it asks for.
use gcp_bigquery_client::client_builder::ClientBuilder;
use gcp_bigquery_client::DRIVE_READONLY_SCOPE;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

const TOKEN_PATH: &str = "/computeMetadata/v1/instance/service-accounts/default/token";

fn scopes_param(request: &Request) -> Option<String> {
    request
        .url
        .query_pairs()
        .find(|(k, _)| k == "scopes")
        .map(|(_, v)| v.into_owned())
}

// Both cases share one test because GCE_METADATA_HOST is process-wide.
#[tokio::test]
async fn workload_identity_requests_additional_scopes_from_metadata_server() {
    let metadata = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(TOKEN_PATH))
        .and(header("Metadata-Flavor", "Google"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": "test-token",
            "expires_in": 3600,
            "token_type": "Bearer"
        })))
        .mount(&metadata)
        .await;
    let bigquery = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .mount(&bigquery)
        .await;

    std::env::set_var("GCE_METADATA_HOST", metadata.address().to_string());

    // Default: no explicit scopes, so the request is exactly what it was before.
    let client = ClientBuilder::new()
        .with_v2_base_url(bigquery.uri())
        .build_with_workload_identity(false)
        .await
        .unwrap();
    let _ = client.dataset().list("project", Default::default()).await;

    // With a Drive scope: BigQuery scope plus Drive, comma-separated.
    let client = ClientBuilder::new()
        .with_v2_base_url(bigquery.uri())
        .with_additional_scopes([DRIVE_READONLY_SCOPE])
        .build_with_workload_identity(true)
        .await
        .unwrap();
    let _ = client.dataset().list("project", Default::default()).await;

    std::env::remove_var("GCE_METADATA_HOST");

    let requests = metadata.received_requests().await.unwrap();
    assert_eq!(requests.len(), 2, "one token request per client");
    assert_eq!(scopes_param(&requests[0]), None);
    assert_eq!(
        scopes_param(&requests[1]).as_deref(),
        Some("https://www.googleapis.com/auth/bigquery.readonly,https://www.googleapis.com/auth/drive.readonly")
    );

    // The token from the metadata server is what reaches BigQuery.
    let bq_requests = bigquery.received_requests().await.unwrap();
    assert_eq!(bq_requests.len(), 2);
    for request in &bq_requests {
        assert_eq!(
            request.headers.get("authorization").unwrap().to_str().unwrap(),
            "Bearer test-token"
        );
    }
}
