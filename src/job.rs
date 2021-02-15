//! Manage BigQuery jobs.
use crate::error::BQError;
use crate::model::job::Job;
use crate::model::job_list::JobList;
use crate::model::query_request::QueryRequest;
use crate::model::query_response::{QueryResponse, ResultSet};
use crate::{process_response, urlencode};
use reqwest::Client;

/// A job API handler.
pub struct JobApi {
    client: Client,
    access_token: String,
}

impl JobApi {
    pub(crate) fn new(client: Client, access_token: String) -> Self {
        Self { client, access_token }
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

        let request = self
            .client
            .post(req_url.as_str())
            .bearer_auth(&self.access_token)
            .json(&query_request)
            .build()?;

        let resp = self.client.execute(request).await?;

        let query_response: QueryResponse = process_response(resp).await?;
        Ok(ResultSet::new(query_response))
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

        let request = self
            .client
            .post(req_url.as_str())
            .bearer_auth(&self.access_token)
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

        let request = self
            .client
            .get(req_url.as_str())
            .bearer_auth(&self.access_token)
            .build()?;

        let resp = self.client.execute(request).await?;

        process_response(resp).await
    }
}

#[cfg(test)]
mod test {
    use crate::error::BQError;
    use crate::model::dataset::Dataset;
    use crate::model::query_request::QueryRequest;
    use crate::model::table::Table;
    use crate::model::table_data_insert_all_request::TableDataInsertAllRequest;
    use crate::model::table_field_schema::TableFieldSchema;
    use crate::model::table_schema::TableSchema;
    use crate::tests::env_vars;
    use crate::Client;
    use serde::Serialize;

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

        let client = Client::new(sa_key).await;

        // Delete the dataset if needed
        let result = client.dataset().delete(project_id, dataset_id, true).await;
        if let Ok(_) = result {
            println!("Removed previous dataset '{}'", dataset_id);
        }

        // Create dataset
        let created_dataset = client.dataset().create(project_id, Dataset::new(dataset_id)).await?;
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

        let created_table = client.table().create(project_id, dataset_id, table).await?;
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

        client
            .tabledata()
            .insert_all(project_id, dataset_id, table_id, insert_request)
            .await?;

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

        client.table().delete(project_id, dataset_id, table_id).await?;

        // Delete dataset
        client.dataset().delete(project_id, dataset_id, true).await?;

        Ok(())
    }
}
