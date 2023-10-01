//! Manage BigQuery jobs.
use std::sync::Arc;

use async_stream::stream;
use reqwest::Client;
use tokio_stream::Stream;

use crate::auth::Authenticator;
use crate::error::BQError;
use crate::model::get_query_results_parameters::GetQueryResultsParameters;
use crate::model::get_query_results_response::GetQueryResultsResponse;
use crate::model::job::Job;
use crate::model::job_cancel_response::JobCancelResponse;
use crate::model::job_configuration::JobConfiguration;
use crate::model::job_configuration_query::JobConfigurationQuery;
use crate::model::job_list::JobList;
use crate::model::job_list_parameters::JobListParameters;
use crate::model::job_reference::JobReference;
use crate::model::query_request::QueryRequest;
use crate::model::query_response::{QueryResponse, ResultSet};
use crate::model::table_row::TableRow;
use crate::{process_response, urlencode, BIG_QUERY_V2_URL};

/// A job API handler.
#[derive(Clone)]
pub struct JobApi {
    client: Client,
    auth: Arc<dyn Authenticator>,
    base_url: String,
}

impl JobApi {
    pub(crate) fn new(client: Client, auth: Arc<dyn Authenticator>) -> Self {
        Self {
            client,
            auth,
            base_url: BIG_QUERY_V2_URL.to_string(),
        }
    }

    pub(crate) fn with_base_url(&mut self, base_url: String) -> &mut Self {
        self.base_url = base_url;
        self
    }

    /// Runs a BigQuery SQL query synchronously and returns query results if the query completes within a specified
    /// timeout.
    /// # Arguments
    /// * `project_id` - Project ID of the query request.
    /// * `query_request` - The request body contains an instance of QueryRequest.
    pub async fn query(&self, project_id: &str, query_request: QueryRequest) -> Result<ResultSet, BQError> {
        let req_url = format!(
            "{base_url}/projects/{project_id}/queries",
            base_url = self.base_url,
            project_id = urlencode(project_id)
        );

        let access_token = self.auth.access_token().await?;

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
    pub fn query_all<'a>(
        &'a self,
        project_id: &'a str,
        query: JobConfigurationQuery,
        page_size: Option<i32>,
    ) -> impl Stream<Item = Result<Vec<TableRow>, BQError>> + 'a {
        stream! {
            let job = Job {
                configuration: Some(JobConfiguration {
                    dry_run: Some(false),
                    query:   Some(query),
                    ..Default::default()
                }),
                ..Default::default()
            };

            let job = self.insert(project_id, job).await?;

            if let Some(ref job_id) = job.job_reference.and_then(|r| r.job_id) {
                let mut page_token: Option<String> = None;
                loop {
                    let qr = self
                        .get_query_results(
                            project_id,
                            job_id,
                            GetQueryResultsParameters {
                                page_token: page_token.clone(),
                                max_results: page_size,
                                ..Default::default()
                            },
                        )
                        .await?;

                    // Waiting for the job to be completed.
                    if !qr.job_complete.unwrap_or(false) {
                        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                        continue;
                    }

                    // Rows is present when the query finishes successfully.
                    // Rows is empty when query result is empty.
                    yield Ok(qr.rows.unwrap_or_default());

                    page_token = match qr.page_token {
                        None => break,
                        f => f,
                    };
                }
            }
        }
    }

    /// Runs a BigQuery SQL query, paginating through all the results synchronously.
    /// Use this function when location of the job differs from the default value (US)
    /// # Arguments
    /// * `project_id`- Project ID of the query request.
    /// * `location`  - Geographic location of the job.
    /// * `query` - The initial query configuration that is submitted when the job is inserted.
    /// * `page_size` - The size of each page fetched. By default, this is set to `None`, and the limit is 10 MB of
    /// rows instead.
    pub fn query_all_with_location<'a>(
        &'a self,
        project_id: &'a str,
        location: &'a str,
        query: JobConfigurationQuery,
        page_size: Option<i32>,
    ) -> impl Stream<Item = Result<Vec<TableRow>, BQError>> + 'a {
        stream! {
            let job = Job {
                configuration: Some(JobConfiguration {
                    dry_run: Some(false),
                    query:   Some(query),
                    ..Default::default()
                }),
                job_reference: Some(JobReference {
                    location:   Some(location.to_string()),
                    project_id: Some(project_id.to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            };

            let job = self.insert(project_id, job).await?;

            if let Some(ref job_id) = job.job_reference.and_then(|r| r.job_id) {
                let mut page_token: Option<String> = None;
                loop {
                    let qr = self
                        .get_query_results(
                            project_id,
                            job_id,
                            GetQueryResultsParameters {
                                page_token: page_token.clone(),
                                max_results: page_size,
                                location:    Some(location.to_string()),
                                ..Default::default()
                            },
                        )
                        .await?;

                        // Waiting for completed the job.
                        if !qr.job_complete.unwrap_or(false) {
                            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

                            continue;
                        }


                    // Rows is present when the query finishes successfully.
                    // Rows be empty when query result is empty.
                    yield Ok(qr.rows.unwrap_or_default());

                    page_token = match qr.page_token {
                        None => break,
                        f => f,
                    };
                }
            }
        }
    }

    /// Runs a BigQuery SQL query, paginating through all the results synchronously.
    /// Use this function when you need to have your job with non-default location, project_id & job_id values
    /// # Arguments
    /// * `project_id`- Project ID of the query request.
    /// * `job_reference` - The initital job reference configuration that is submitted when the job is inserted
    /// * `query` - The initial query configuration that is submitted when the job is inserted.
    /// * `page_size` - The size of each page fetched. By default, this is set to `None`, and the limit is 10 MB of
    /// rows instead.
    pub fn query_all_with_job_reference<'a>(
        &'a self,
        project_id: &'a str,
        job_reference: JobReference,
        query: JobConfigurationQuery,
        page_size: Option<i32>,
    ) -> impl Stream<Item = Result<Vec<TableRow>, BQError>> + 'a {
        stream! {
            let location = job_reference.location.as_ref().cloned();

            let job = Job {
                configuration: Some(JobConfiguration {
                    dry_run: Some(false),
                    query:   Some(query),
                    ..Default::default()
                }),
                job_reference: Some(job_reference),
                ..Default::default()
            };

            let job = self.insert(project_id, job).await?;

            if let Some(ref job_id) = job.job_reference.and_then(|r| r.job_id) {
                let mut page_token: Option<String> = None;
                loop {
                    let gqrp = GetQueryResultsParameters {
                                page_token,
                                max_results: page_size,
                                location:    location.clone(),
                                ..Default::default()
                            };
                    let qr = self
                        .get_query_results(
                            project_id,
                            job_id,
                            gqrp,
                        )
                        .await?;

                    // Rows is present when the query finishes successfully.
                    yield Ok(qr.rows.expect("Rows are not present"));

                    page_token = match qr.page_token {
                        None => break,
                        f => f,
                    };
                }
            }
        }
    }

    /// Starts a new asynchronous job.
    /// # Arguments
    /// * `project_id` - Project ID of project that will be billed for the job.
    /// * `job` - The request body contains an instance of Job.
    pub async fn insert(&self, project_id: &str, job: Job) -> Result<Job, BQError> {
        let req_url = format!(
            "{base_url}/projects/{project_id}/jobs",
            base_url = self.base_url,
            project_id = urlencode(project_id)
        );

        let access_token = self.auth.access_token().await?;

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
            "{base_url}/projects/{project_id}/jobs",
            base_url = self.base_url,
            project_id = urlencode(project_id)
        );

        let access_token = self.auth.access_token().await?;

        let request = self.client.get(req_url.as_str()).bearer_auth(access_token).build()?;

        let resp = self.client.execute(request).await?;

        process_response(resp).await
    }

    /// Lists all jobs that you started in the specified project paginating through all the results synchronously.
    /// Job information is available for a six month period after creation.
    /// The job list is sorted in reverse chronological order, by job creation time. Requires the Can
    /// View project role, or the Is Owner project role if you set the allUsers property.
    /// # Arguments
    /// * `project_id` - Project ID of the jobs to list.
    /// * `parameters` - The query parameters for jobs.list.
    pub fn get_job_list<'a>(
        &'a self,
        project_id: &'a str,
        parameters: Option<JobListParameters>,
    ) -> impl Stream<Item = Result<JobList, BQError>> + 'a {
        stream! {
            let req_url = format!(
                "{base_url}/projects/{project_id}/jobs",
                base_url = self.base_url,
                project_id = urlencode(project_id),
                );
            let mut params = parameters.unwrap_or_default();
            let mut page_token: Option<String> = None;
            loop {
                let mut request_builder = self.client.get(req_url.as_str());

                params.page_token = page_token;
                request_builder = request_builder.query(&params);

                let access_token = self.auth.access_token().await?;
                let request = request_builder.bearer_auth(access_token).build()?;

                let resp = self.client.execute(request).await?;

                let process_resp: Result<JobList, BQError> = process_response(resp).await;

                yield match process_resp {
                    Err(e) => {page_token=None; Err(e)},
                    Ok(job_list) => {page_token=job_list.next_page_token.clone(); Ok(job_list.clone())}
                };

                if page_token.is_none() {
                    break;
                }
            }
        }
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
            "{base_url}/projects/{project_id}/queries/{job_id}",
            base_url = self.base_url,
            project_id = urlencode(project_id),
            job_id = urlencode(job_id),
        );

        let access_token = self.auth.access_token().await?;

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
            "{base_url}/projects/{project_id}/jobs/{job_id}",
            base_url = self.base_url,
            project_id = urlencode(project_id),
            job_id = urlencode(job_id),
        );

        let mut request_builder = self.client.get(req_url.as_str());

        if let Some(location) = location {
            request_builder = request_builder.query(&[("location", location)]);
        }

        let access_token = self.auth.access_token().await?;
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
            "{base_url}/projects/{project_id}/jobs/{job_id}/cancel",
            base_url = self.base_url,
            project_id = urlencode(project_id),
            job_id = urlencode(job_id),
        );

        let mut request_builder = self.client.post(req_url.as_str());

        if let Some(location) = location {
            request_builder = request_builder.query(&[("location", location)]);
        }

        let access_token = self.auth.access_token().await?;

        let request = request_builder.bearer_auth(access_token).build()?;

        let resp = self.client.execute(request).await?;

        process_response(resp).await
    }
}

#[cfg(test)]
mod test {
    use serde::Serialize;
    use tokio_stream::StreamExt;

    use crate::error::BQError;
    use crate::model::dataset::Dataset;
    use crate::model::field_type::serialize_json_as_string;
    use crate::model::job_configuration_query::JobConfigurationQuery;
    use crate::model::job_reference::JobReference;
    use crate::model::query_parameter::QueryParameter;
    use crate::model::query_parameter_type::QueryParameterType;
    use crate::model::query_parameter_value::QueryParameterValue;
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
        // Serialized as string but deserialized as serde json value.
        #[serde(serialize_with = "serialize_json_as_string")]
        json_value: serde_json::value::Value,
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
        let dataset_id = &format!("{dataset_id}_job");

        let client = Client::from_service_account_key_file(sa_key).await?;

        client.table().delete_if_exists(project_id, dataset_id, table_id).await;
        client.dataset().delete_if_exists(project_id, dataset_id, true).await;

        // Create dataset
        let created_dataset = client.dataset().create(Dataset::new(project_id, dataset_id)).await?;
        assert_eq!(created_dataset.id, Some(format!("{project_id}:{dataset_id}")));

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
                TableFieldSchema::json("json_value"),
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
                json_value: serde_json::from_str("{\"a\":2,\"b\":\"hello\"}")?,
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
                json_value: serde_json::from_str("{\"a\":1,\"b\":\"goodbye\",\"c\":3}")?,
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
                json_value: serde_json::from_str("{\"b\":\"world\",\"c\":2}")?,
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
                json_value: serde_json::from_str("{\"a\":3,\"c\":1}")?,
            },
        )?;

        let n_rows = insert_request.len();

        let result = client
            .tabledata()
            .insert_all(project_id, dataset_id, table_id, insert_request)
            .await;

        assert!(result.is_ok(), "{:?}", result);
        let result = result.unwrap();
        assert!(result.insert_errors.is_none(), "{:?}", result);

        // Query
        let mut rs = client
            .job()
            .query(
                project_id,
                QueryRequest::new(format!(
                    "SELECT COUNT(*) AS c FROM `{project_id}.{dataset_id}.{table_id}`"
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

        // Query all
        let query_all_results: Result<Vec<_>, _> = client
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
            .collect::<Result<Vec<_>, _>>()
            .await
            .map(|vec_of_vecs| vec_of_vecs.into_iter().flatten().collect());

        assert!(query_all_results.is_ok());
        assert_eq!(query_all_results.unwrap().len(), n_rows);

        // Query all with location
        let location = "us";
        let query_all_results_with_location: Result<Vec<_>, _> = client
            .job()
            .query_all_with_location(
                project_id,
                location,
                JobConfigurationQuery {
                    query: format!("SELECT * FROM `{project_id}.{dataset_id}.{table_id}`"),
                    query_parameters: None,
                    use_legacy_sql: Some(false),
                    ..Default::default()
                },
                Some(2),
            )
            .collect::<Result<Vec<_>, _>>()
            .await
            .map(|vec_of_vecs| vec_of_vecs.into_iter().flatten().collect());

        assert!(query_all_results_with_location.is_ok());
        assert_eq!(query_all_results_with_location.unwrap().len(), n_rows);

        // Query all with JobReference
        let job_reference = JobReference {
            project_id: Some(project_id.to_string()),
            location: Some(location.to_string()),
            ..Default::default()
        };
        let query_all_results_with_job_reference: Result<Vec<_>, _> = client
            .job()
            .query_all_with_job_reference(
                project_id,
                job_reference,
                JobConfigurationQuery {
                    query: format!("SELECT * FROM `{project_id}.{dataset_id}.{table_id}`"),
                    query_parameters: None,
                    use_legacy_sql: Some(false),
                    ..Default::default()
                },
                Some(2),
            )
            .collect::<Result<Vec<_>, _>>()
            .await
            .map(|vec_of_vecs| vec_of_vecs.into_iter().flatten().collect());

        assert!(query_all_results_with_job_reference.is_ok());
        assert_eq!(query_all_results_with_job_reference.unwrap().len(), n_rows);

        // Query all with json parameter
        let query_all_results_with_parameter: Result<Vec<_>, _> = client
            .job()
            .query_all(
                project_id,
                JobConfigurationQuery {
                    query: format!("SELECT int_value, json_value.a, json_value.b FROM `{project_id}.{dataset_id}.{table_id}` where CAST(JSON_VALUE(json_value,'$.a') as int) >= @compare"),
                    query_parameters: Some(vec![QueryParameter {
                        name: Some("compare".to_string()),
                        parameter_type: Some(QueryParameterType { array_type: None, struct_types: None, r#type: "INTEGER".to_string() }),
                        parameter_value: Some(QueryParameterValue { array_values: None, struct_values: None, value: Some("2".to_string()) }),
                    }]),
                    use_legacy_sql: Some(false),
                    ..Default::default()
                },
                Some(2),
            )
            .collect::<Result<Vec<_>, _>>()
            .await
            .map(|vec_of_vecs| vec_of_vecs.into_iter().flatten().collect());

        assert!(query_all_results_with_parameter.is_ok());
        // 2 rows match the query: {"a":2,"b":"hello"} and {"a":3,"c":1}
        assert_eq!(query_all_results_with_parameter.unwrap().len(), 2);

        // Delete table
        client.table().delete(project_id, dataset_id, table_id).await?;

        // Delete dataset
        client.dataset().delete(project_id, dataset_id, true).await?;

        Ok(())
    }
}
