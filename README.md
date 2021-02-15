GCP BigQuery Client (Rust)
==========================

[<img alt="github" src="https://img.shields.io/badge/github-lquerel/gcp_bigquery_client-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/lquerel/gcp-bigquery-client)
[<img alt="crates.io" src="https://img.shields.io/crates/v/gcp_bigquery_client.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/gcp-bigquery-client)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-gcp_bigquery_client-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/gcp-bigquery-client)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/lquerel/gcp-bigquery-client/Rust/main?style=for-the-badge" height="20">](https://github.com/lquerel/gcp-bigquery-client/actions?query=branch%3Amain)

An ergonomic async client library for GCP BigQuery.
* Support for dataset, table, streaming API and query (see [status section](#status) for an exhaustive list of supported API endpoints)
* Support Service Account Key authentication (other OAuth flows will be added later)
* Create tables and rows via builder patterns
* Persist complex Rust structs in structured BigQuery tables
* Async API

<br>

Other OAuth flows will be added later. 

---
**NOTE**
This is my first crate and it's still a **work-in-progress**. So please post your suggestions and ideas on this 
GitHub [discussion section](https://github.com/lquerel/gcp-bigquery-client/discussions).  

Most of the Rust structures defined in the directory 'model' are derived from this [Google API Explorer document](https://bigquery.googleapis.com/discovery/v1/apis/bigquery/v2/rest). 

---

## Example
The following example performs the following operations:
* Load a set of environment variables to set `$PROJECT_ID`, `$DATASET_ID`, `$TABLE_ID` and `$GOOGLE_APPLICATION_CREDENTIALS`
* Init the BigQuery client
* Create a dataset in the GCP project `$PROJECT_ID`
* Create a table in the previously created dataset (table schema)
* Insert a set of rows in the previously created table via the BigQuery Streaming API. The inserted 
rows are based on a regular Rust struct implementing the trait Serialize. 
* Perform a select query on the previously created table
* Drop the table previously created
* Drop the dataset previously created 

```rust
// Read configuration parameters from environment variables
let (ref project_id, ref dataset_id, ref table_id, ref gcp_sa_key) = env_vars();

// Init BigQuery client
let client = gcp_bigquery_client::Client::new(gcp_sa_key).await;

// Create dataset
let created_dataset = client.dataset().create(project_id, Dataset::new(dataset_id)).await?;
println!(
    "Dataset '{}.{}' created",
    created_dataset.project_id(),
    created_dataset.dataset_id()
);

// Create table schema
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
println!(
    "Table '{}.{}.{}' created",
    created_table.project_id(),
    created_table.dataset_id(),
    created_table.table_id()
);

// Insert data via BigQuery Streaming API
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
    println!("Number of rows inserted: {}", rs.get_i64_by_name("c")?.unwrap());
}

// Delete the table previously created
client.table().delete(project_id, dataset_id, table_id).await?;

// Delete the dataset previously created
client.dataset().delete(project_id, dataset_id, true).await?;
```

## Main dependencies
* Yup-OAuth2 5.x
* Hyper 0.14 (+ RusTLS)
* Tokio 1.x
* Reqwest 0.11
* Serde JSON 1.x
* ThisError 1.x

## Status
List of endpoints implemented:
- [X] Dataset
  - [X] Delete
  - [X] Get
  - [X] Insert (create)
  - [X] List
  - [X] Patch
  - [X] Update
- [X] Table
  - [X] Delete
  - [X] Get
  - [X] GetIamPolicy
  - [X] Insert
  - [X] List
  - [X] Patch
  - [X] SetIamPolicy
  - [X] TestIamPermissions
  - [X] Update
- [ ] Tabledata 
  - [X] InsertAll
  - [ ] List
- [ ] Job
  - [ ] Cancel
  - [ ] Get
  - [ ] GetQueryResult
  - [ ] Insert
  - [ ] List
  - [X] Query 
- [ ] Model
  - [ ] Delete
  - [ ] Get
  - [ ] List
  - [ ] Patch
- [ ] Project
  - [ ] GetServiceAccount
  - [ ] List 
- [ ] Routine  
  - [ ] Delete
  - [ ] Get
  - [ ] Insert
  - [ ] List
  - [ ] Update

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>