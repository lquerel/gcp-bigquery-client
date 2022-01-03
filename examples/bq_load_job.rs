use cloud_storage::Object;
use gcp_bigquery_client::model::job::Job;
use gcp_bigquery_client::model::job_configuration::JobConfiguration;
use gcp_bigquery_client::model::job_configuration_load::JobConfigurationLoad;
use gcp_bigquery_client::model::job_reference::JobReference;
use gcp_bigquery_client::model::job_status::JobStatus;
use gcp_bigquery_client::model::table_reference::TableReference;
use gcp_bigquery_client::Client;

/// This example explains how to initiate and supervise a BQ load job (new line delimited json file in GCS).
use std::env;
use std::thread::sleep;
use std::time::Duration;

const GCS_BUCKET_NAME: &'static str = "rust_bq_client";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (gcp_sa_key, project_id) = env_vars();

    // Create temporary file name
    let tmp_file_name = tmp_file_name(30);

    // Load line delimiter json data file
    let data = std::fs::read("examples/data.json").expect("data.json not found");

    // Store data on GCS
    let source_uri = store_and_get_gcs_uri(GCS_BUCKET_NAME, data, &tmp_file_name).await?;

    let client = Client::from_service_account_key_file(&gcp_sa_key).await;

    // Create BQ load job to create/update the test with the content of the json data file
    // Pre-requisite: test_batch_load dataset already created
    let job_ref = create_bq_load_job(
        &client,
        &project_id,
        "test_batch_load",
        "test4",
        source_uri,
        &tmp_file_name,
    )
    .await?;

    while get_job_status(&client, &project_id, job_ref.job_id.as_ref().unwrap())
        .await?
        .state
        != Some("DONE".to_string())
    {
        sleep(Duration::from_secs(1));
    }

    println!("DONE");
    Ok(())
}

pub async fn create_bq_load_job(
    client: &gcp_bigquery_client::Client,
    project_id: &str,
    dataset_id: &str,
    table_id: &str,
    source_uri: String,
    tmp_file_name: &str,
) -> Result<JobReference, Box<dyn std::error::Error>> {
    let job = Job {
        configuration: Some(JobConfiguration {
            job_timeout_ms: Some("30000".to_string()),
            load: Some(JobConfigurationLoad {
                allow_jagged_rows: Some(true),
                autodetect: Some(true),
                // Never allow this job to create new tables.
                create_disposition: Some("CREATE_IF_NEEDED".to_string()),
                destination_table: Some(TableReference::new(project_id, dataset_id, table_id)),
                // Default to JSON for now
                json_extension: None,
                // None = no bad records are allowed.
                max_bad_records: None,
                source_format: Some("NEWLINE_DELIMITED_JSON".to_string()),
                source_uris: Some(vec![source_uri]),
                ..Default::default()
            }),
            ..Default::default()
        }),
        job_reference: Some(JobReference {
            job_id: Some(tmp_file_name.into()),
            project_id: Some(project_id.into()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let job = client.job().insert(project_id, job).await?;

    Ok(job.job_reference.expect("job_reference not found"))
}

pub async fn get_job_status(
    client: &gcp_bigquery_client::Client,
    project_id: &str,
    job_id: &str,
) -> Result<JobStatus, Box<dyn std::error::Error>> {
    let job = client.job().get_job(project_id, job_id, None).await?;

    Ok(job.status.expect("job_status not found"))
}

fn tmp_file_name(file_name_len: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let mut rng = rand::thread_rng();

    (0..file_name_len)
        .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
        .collect()
}

pub async fn store_and_get_gcs_uri(
    gcs_bucket_name: &str,
    data: Vec<u8>,
    file_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let object = Object::create(gcs_bucket_name, data, file_name, "application/text").await?;

    Ok(format!("gs://{}/{}", object.bucket, object.name))
}

pub fn env_vars() -> (String, String) {
    let project_id = env::var("PROJECT_ID").expect("Environment variable PROJECT_ID");
    let gcp_sa_key =
        env::var("GOOGLE_APPLICATION_CREDENTIALS").expect("Environment variable GOOGLE_APPLICATION_CREDENTIALS");

    (gcp_sa_key, project_id)
}
