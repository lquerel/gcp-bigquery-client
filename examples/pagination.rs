use gcp_bigquery_client::{env_vars, model::job_configuration_query::JobConfigurationQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (ref project_id, ref dataset_id, ref table_id, ref gcp_sa_key) = env_vars();
    let client = gcp_bigquery_client::Client::from_service_account_key_file(gcp_sa_key).await;

    let result_set = client
        .job()
        .query_all(
            project_id,
            JobConfigurationQuery {
                query: format!("SELECT * FROM `{project_id}.{dataset_id}.{table_id}`"),
                query_parameters: None,
                use_legacy_sql: Some(false),
                ..Default::default()
            },
            Some(1),
        )
        .await?;

    println!("{}", result_set.rows().len());

    Ok(())
}
