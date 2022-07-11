//! Manage BigQuery jobs.
use reqwest::Client;

use crate::auth::ServiceAccountAuthenticator;
use crate::error::BQError;
use crate::model::get_query_results_parameters::GetQueryResultsParameters;
use crate::model::get_query_results_response::{GetQueryResultsResponse, PaginatedResultSet};
use crate::model::job::Job;
use crate::model::job_cancel_response::JobCancelResponse;
use crate::model::job_configuration::JobConfiguration;
use crate::model::job_configuration_query::JobConfigurationQuery;
use crate::model::job_list::JobList;
use crate::model::query_request::QueryRequest;
use crate::model::query_response::{QueryResponse, ResultSet};
use crate::{process_response, urlencode};

/// A job API handler.
#[derive(Clone)]
pub struct JobApi {
    client: Client,
    sa_auth: ServiceAccountAuthenticator,
}

impl JobApi {
    pub(crate) fn new(client: Client, sa_auth: ServiceAccountAuthenticator) -> Self {
        Self { client, sa_auth }
    }

    /// Runs a BigQuery SQL query synchronously and returns query results if the query completes within a specified
    /// timeout.
    /// # Arguments
    /// * `project_id` - Project ID of the query request.
    /// * `query_request` - The request body contains an instance of QueryRequest.
    pub async fn query(&self, project_id: &str, query_request: QueryRequest) -> Result<ResultSet, BQError> {
        let req_url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/queries",
            project_id = urlencode(project_id)
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self
            .client
            .post(req_url.as_str())
            .bearer_auth(access_token)
            .json(&query_request)
            .build()?;

        let resp = self.client.execute(request).await?;

        let query_response: QueryResponse = process_response(resp).await?;
        Ok(ResultSet::new(query_response))
    }

    /// Runs a BigQuery SQL query, paginating through all the results synchronously.
    /// # Arguments
    /// * `project_id`- Project ID of the query request.
    /// * `query` - The initial query configuration that is submitted when the job is inserted.
    /// * `page_size` - The size of each page fetched. By default, this is set to `None`, and the limit is 10 MB of
    /// rows instead.
    pub async fn query_all(
        &self,
        project_id: &str,
        query: JobConfigurationQuery,
        page_size: Option<i32>,
    ) -> Result<PaginatedResultSet, BQError> {
        let job = Job {
            configuration: Some(JobConfiguration {
                dry_run: Some(false),
                query: Some(query),
                ..Default::default()
            }),
            ..Default::default()
        };

        let mut rs = PaginatedResultSet::default();

        let job = self.insert(project_id, job).await?;
        if let Some(ref job_id) = job.job_reference.and_then(|r| r.job_id) {
            let mut page_token: Option<String> = None;
            loop {
                let qr = self
                    .get_query_results(
                        project_id,
                        job_id,
                        GetQueryResultsParameters {
                            page_token,
                            max_results: page_size,
                            ..Default::default()
                        },
                    )
                    .await?;

                rs.append(qr.rows);

                if qr.page_token.is_none() {
                    break;
                }
                page_token = qr.page_token;
            }
        }

        Ok(rs)
    }

    /// Starts a new asynchronous job.
    /// # Arguments
    /// * `project_id` - Project ID of project that will be billed for the job.
    /// * `job` - The request body contains an instance of Job.
    pub async fn insert(&self, project_id: &str, job: Job) -> Result<Job, BQError> {
        let req_url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/jobs",
            project_id = urlencode(project_id)
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self
            .client
            .post(req_url.as_str())
            .bearer_auth(access_token)
            .json(&job)
            .build()?;

        let resp = self.client.execute(request).await?;

        process_response(resp).await
    }

    /// Lists all jobs that you started in the specified project. Job information is available for a six month period
    /// after creation. The job list is sorted in reverse chronological order, by job creation time. Requires the Can
    /// View project role, or the Is Owner project role if you set the allUsers property.
    /// # Arguments
    /// * `project_id` - Project ID of the jobs to list.
    pub async fn list(&self, project_id: &str) -> Result<JobList, BQError> {
        let req_url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/jobs",
            project_id = urlencode(project_id)
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self.client.get(req_url.as_str()).bearer_auth(access_token).build()?;

        let resp = self.client.execute(request).await?;

        process_response(resp).await
    }

    /// RPC to get the results of a query job.
    /// # Arguments
    /// * `project_id` - Project ID of the query request.
    /// * `job_id` - Job ID of the query job.
    /// * `parameters` - The query parameters for jobs.getQueryResults.
    pub async fn get_query_results(
        &self,
        project_id: &str,
        job_id: &str,
        parameters: GetQueryResultsParameters,
    ) -> Result<GetQueryResultsResponse, BQError> {
        let req_url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/queries/{job_id}",
            project_id = urlencode(project_id),
            job_id = urlencode(job_id),
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self
            .client
            .get(req_url.as_str())
            .query(&parameters)
            .bearer_auth(access_token)
            .build()?;

        let resp = self.client.execute(request).await?;

        let get_query_results_response: GetQueryResultsResponse = process_response(resp).await?;
        Ok(get_query_results_response)
    }

    /// Returns information about a specific job. Job information is available for a six month
    /// period after creation. Requires that you're the person who ran the job, or have the Is
    /// Owner project role.
    /// # Arguments
    /// * `project_id` - Project ID of the requested job.
    /// * `job_id` - Job ID of the requested job.
    /// * `location` - The geographic location of the job. Required except for US and EU. See
    /// details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location.
    pub async fn get_job(&self, project_id: &str, job_id: &str, location: Option<&str>) -> Result<Job, BQError> {
        let req_url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/jobs/{job_id}",
            project_id = urlencode(project_id),
            job_id = urlencode(job_id),
        );

        let mut request_builder = self.client.get(req_url.as_str());

        if let Some(location) = location {
            request_builder = request_builder.query(&[("location", location)]);
        }

        let access_token = self.sa_auth.access_token().await?;
        let request = request_builder.bearer_auth(access_token).build()?;

        let resp = self.client.execute(request).await?;

        process_response(resp).await
    }

    /// Requests that a job be cancelled. This call will return immediately, and the client will
    /// need to poll for the job status to see if the cancel completed successfully. Cancelled jobs
    /// may still incur costs.
    /// # Arguments
    /// * `project_id` - Project ID of the job to cancel.
    /// * `job_id` - Job ID of the job to cancel.
    /// * `location` - The geographic location of the job. Required except for US and EU. See
    /// details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location.
    pub async fn cancel_job(
        &self,
        project_id: &str,
        job_id: &str,
        location: Option<&str>,
    ) -> Result<JobCancelResponse, BQError> {
        let req_url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/jobs/{job_id}/cancel",
            project_id = urlencode(project_id),
            job_id = urlencode(job_id),
        );

        let mut request_builder = self.client.post(req_url.as_str());

        if let Some(location) = location {
            request_builder = request_builder.query(&[("location", location)]);
        }

        let access_token = self.sa_auth.access_token().await?;

        let request = request_builder.bearer_auth(access_token).build()?;

        let resp = self.client.execute(request).await?;

        process_response(resp).await
    }
}

#[cfg(test)]
mod test {
    use serde::Serialize;

    use crate::error::BQError;
    use crate::model::dataset::Dataset;
    use crate::model::job_configuration_query::JobConfigurationQuery;
    use crate::model::query_request::QueryRequest;
    use crate::model::query_response::{QueryResponse, ResultSet};
    use crate::model::table::Table;
    use crate::model::table_data_insert_all_request::TableDataInsertAllRequest;
    use crate::model::table_field_schema::TableFieldSchema;
    use crate::model::table_schema::TableSchema;
    use crate::{env_vars, Client};

    #[derive(Serialize)]
    struct MyRow {
        int_value: i64,
        float_value: f64,
        bool_value: bool,
        string_value: String,
        record_value: FirstRecordLevel,
    }

    #[derive(Serialize)]
    struct FirstRecordLevel {
        int_value: i64,
        string_value: String,
        record_value: SecondRecordLevel,
    }

    #[derive(Serialize)]
    struct SecondRecordLevel {
        int_value: i64,
        string_value: String,
    }

    #[tokio::test]
    async fn test() -> Result<(), BQError> {
        let (ref project_id, ref dataset_id, ref table_id, ref sa_key) = env_vars();
        let dataset_id = &format!("{}_job", dataset_id);

        let client = Client::from_service_account_key_file(sa_key).await;

        client.table().delete_if_exists(project_id, dataset_id, table_id).await;
        client.dataset().delete_if_exists(project_id, dataset_id, true).await;

        // Create dataset
        let created_dataset = client.dataset().create(Dataset::new(project_id, dataset_id)).await?;
        assert_eq!(created_dataset.id, Some(format!("{}:{}", project_id, dataset_id)));

        // Create table
        let table = Table::new(
            project_id,
            dataset_id,
            table_id,
            TableSchema::new(vec![
                TableFieldSchema::integer("int_value"),
                TableFieldSchema::float("float_value"),
                TableFieldSchema::bool("bool_value"),
                TableFieldSchema::string("string_value"),
                TableFieldSchema::record(
                    "record_value",
                    vec![
                        TableFieldSchema::integer("int_value"),
                        TableFieldSchema::string("string_value"),
                        TableFieldSchema::record(
                            "record_value",
                            vec![
                                TableFieldSchema::integer("int_value"),
                                TableFieldSchema::string("string_value"),
                            ],
                        ),
                    ],
                ),
            ]),
        );

        let created_table = client.table().create(table).await?;
        assert_eq!(created_table.table_reference.table_id, table_id.to_string());

        // Insert data
        let mut insert_request = TableDataInsertAllRequest::new();
        insert_request.add_row(
            None,
            MyRow {
                int_value: 1,
                float_value: 1.0,
                bool_value: false,
                string_value: "first".into(),
                record_value: FirstRecordLevel {
                    int_value: 10,
                    string_value: "sub_level_1.1".into(),
                    record_value: SecondRecordLevel {
                        int_value: 20,
                        string_value: "leaf".to_string(),
                    },
                },
            },
        )?;
        insert_request.add_row(
            None,
            MyRow {
                int_value: 2,
                float_value: 2.0,
                bool_value: true,
                string_value: "second".into(),
                record_value: FirstRecordLevel {
                    int_value: 11,
                    string_value: "sub_level_1.2".into(),
                    record_value: SecondRecordLevel {
                        int_value: 21,
                        string_value: "leaf".to_string(),
                    },
                },
            },
        )?;
        insert_request.add_row(
            None,
            MyRow {
                int_value: 3,
                float_value: 3.0,
                bool_value: false,
                string_value: "third".into(),
                record_value: FirstRecordLevel {
                    int_value: 12,
                    string_value: "sub_level_1.3".into(),
                    record_value: SecondRecordLevel {
                        int_value: 22,
                        string_value: "leaf".to_string(),
                    },
                },
            },
        )?;
        insert_request.add_row(
            None,
            MyRow {
                int_value: 4,
                float_value: 4.0,
                bool_value: true,
                string_value: "fourth".into(),
                record_value: FirstRecordLevel {
                    int_value: 13,
                    string_value: "sub_level_1.4".into(),
                    record_value: SecondRecordLevel {
                        int_value: 23,
                        string_value: "leaf".to_string(),
                    },
                },
            },
        )?;

        let n_rows = insert_request.len();

        let result = client
            .tabledata()
            .insert_all(project_id, dataset_id, table_id, insert_request)
            .await;

        assert!(result.is_ok(), "{:?}", result);

        // Query
        let mut rs = client
            .job()
            .query(
                project_id,
                QueryRequest::new(format!(
                    "SELECT COUNT(*) AS c FROM `{}.{}.{}`",
                    project_id, dataset_id, table_id
                )),
            )
            .await?;
        while rs.next_row() {
            assert!(rs.get_i64_by_name("c")?.is_some());
        }

        // Get job id
        let job_id = rs
            .query_response()
            .job_reference
            .as_ref()
            .expect("expected job_reference")
            .job_id
            .clone()
            .expect("expected job_id");

        let job = client.job_api.get_job(project_id, &job_id, None).await?;
        assert_eq!(job.status.unwrap().state.unwrap(), "DONE");

        // GetQueryResults
        let query_results = client
            .job()
            .get_query_results(project_id, &job_id, Default::default())
            .await?;
        let mut query_results_rs = ResultSet::new(QueryResponse::from(query_results));
        assert_eq!(query_results_rs.row_count(), rs.row_count());
        while query_results_rs.next_row() {
            assert!(rs.get_i64_by_name("c")?.is_some());
        }

        //Query all
        let query_all_results = client
            .job()
            .query_all(
                project_id,
                JobConfigurationQuery {
                    query: format!("SELECT * FROM `{project_id}.{dataset_id}.{table_id}`"),
                    query_parameters: None,
                    use_legacy_sql: Some(false),
                    ..Default::default()
                },
                Some(2),
            )
            .await?;

        assert_eq!(query_all_results.rows().len(), n_rows);

        client.table().delete(project_id, dataset_id, table_id).await?;

        // Delete dataset
        client.dataset().delete(project_id, dataset_id, true).await?;

        Ok(())
    }
}
