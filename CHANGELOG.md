# Changelog

All notable changes to this project will be documented in this file.

## [0.23.0] - 2024-08-28

#### Changed

- Added a new method `ResultSet::new_from_get_query_results_response` which creates a `ResultSet` from a `GetQueryResultsResponse`.

- Breaking changes:

  - Return type of `JobApi::query` changed from `Result<ResultSet, BQError>` to `Result<QueryResponse, BQError>`.
  - `ResultSet::new` renamed to `ResultSet::new_from_query_response`.

- Rationale for the breaking changes:

  `JobApi::query` now returns `Result<QueryResponse, BQError>` instead of `Result<ResultSet, BQError>`. A `ResultSet` wraps over a `QueryResponse` but callers didn't have acces to that internal object. To allow callers access to the internal object, `JobApi::query` now returns the internal object itself. This means older code which expected a `ResultSet` will break.

- Upgrading to the new version:

  To fix broken code, you'll have to call `ResultSet::from_query_response` function. For example, if your code looked like this:

  ```rust
  let mut result_set = client
        .job()
        .query(
            project_id,
            query_request,
        )
        .await?;
  ```

  It should be updated to:

  ```rust
  let query_response = client
        .job()
        .query(
            project_id,
            query_request,
        )
        .await?;
    let mut result_set = ResultSet::new_from_query_response(query_response);
  ```

  Another reason for the change was making it consistent with `JobApi::get_query_results` which already returned an unwrapped object which callers needed to manually wrap inside a `ResultSet` by calling `ResultSet::new` method.

## [0.23.0] - 2024-08-10

### Fix

- Fix a root certificate issue following a breaking change in Tonic (by @imor).
- Remove dependency on protoc. The generated code is now included in the repository (by @lquerel).

### Maintenance

- Bump versions of `thiserror`, `hyper-util`, `tokio`, `serde`, `serde_json`, `tonic`, `tonic-build`.

## [0.22.0] - 2024-07-07

### Added

- Add partial support for BigQuery Storage Write API (by @imor).
  - append_rows
  - get_write_stream
- Add GZIP support for `insert_all` (by @Deniskore). The `gzip` feature is included by default.
  See https://github.com/lquerel/gcp-bigquery-client/issues/74 for more information.

Breaking changes:

- Client::from_authenticator is now async.
- ClientBuilder::build_from_authenticator is now async.

### Maintenance

- Bump version of `yup_oauth2` to v11 (by @serpex). `gcp-bigquery-client` can now be used with ring or aws-lc-rs as
  a TLS backend.
- Fix Clippy warnings produced with Rust 1.78.0 (by @imor).

## [0.21.0] - 2024-06-16

### Maintenance

- Bump version of yup_oauth2 to 10.0.1 (by @serpex).

## [0.20.0] - 2024-04-28

### Maintenance

- Re-export yup_oauth2 as suggested by @serpex.

## [0.19.0] - 2024-04-27

### Maintenance

- Re-export hyper_rustls from yup (Thanks to @DoumanAsh)
- Bump versions of all dependencies to their most recent versions (including hyper and yup-oauth2)

## [0.18.1] - 2024-03-29

### Improvement

- Add support to bigquery-emulator (Thanks to @henriiik)
- Add support to use the ClientBuilder to build a Client with an Authenticator (Thanks to @henriiik)

### Fix

- Fix build issue with hyper-rustls (Thanks to @OmriSteiner and @nate-kelley-buster)

## [0.18.0] - 2023-10-22

### Fix

- Fix tabledata.list return type (Thank you @enricozb)

## [0.17.1] - 2023-10-01

### Improvement

- Feature/paginate through all jobs list #63 (Thank you @Shirlo)

### Fix

- Apply the same fix as #54 to query_all #64 (Thank you @LawnGnome)

## [0.17.0] - 2023-07-01

### Improvement

- Add support for JSON column types. Thanks to Michael Gibson (@mchlgibs) for this contribution.

## [0.16.8] - 2023-06-11

### Improvement

- Add `get_serde` and `get_serde_by_name` methods to read columns for any types that implements `serde::de:DeserializeOwned`. Thanks to @andyquinterom for this contribution.

## [0.16.7] - 2023-04-25

### Improvement

- Update hyper-rustls version.

## [0.16.6] - 2023-03-06

### Improvement

- Fix Rows are not present when job not completed yet (Thanks @alu).

## [0.16.5] - 2023-02-05

### Improvement

- Add `format_options` field to `QueryRequest` (Thanks @JichaoS).

## [0.16.4] - 2022-12-28

### Improvement

- Update `FieldType` enum to `Geography` field type (Thanks @kiibo382).

## [0.16.3] - 2022-12-6

### Improvement

- Two new methods in the job API `query_all_with_location` and `query_all_with_job_reference` (Thanks @MikhailMS).

## [0.16.2] - 2022-11-12

### Improvement

- Example to run with bigquery-emulator (Thanks @marcoleni).

## [0.16.1] - 2022-11-09

### Improvement

- Application default credentials auth & authorized user auth (#46) (Thanks @kiibo382).

### Fix

- Installed flow auth - execute the authorization code flow before returning (#45) (Thanks @kiibo382).
- Avoid panic when schema doesn't exist (#44) (Thanks @lee-hen).

## [0.16.0] - 2022-11-06

### Improvement

- Better error management in [Client::from_service_account_key_file](https://github.com/lquerel/gcp-bigquery-client/issues/39) (Thanks @burmecia).

## [0.15.0] - 2022-11-02

### Improvement

- Abstraction of the Authenticator (Thanks @kiibo382).
- InstalledFlowAuthenticator support (Thanks @kiibo382).
- Support for BigQuery emulator (Thanks @Marcoleni)
- Bump yup-oauth2 crate version.
- Bump time crate version (security issue).

## [0.14.0] - 2022-09-27

### Fix

- Process list - Change numeric_id type from u64 to string (Thanks @kiibo382)

## [0.13.0] - 2022-07-11

### Improvement

- Make Client cloneable (Thanks @lee-hen).
- Bump yup-oauth2 crate version.
- Bump time crate version (security issue).
- Change Job.query_all method signature to return a stream to support more use cases (Thanks LiHRaM).

## [0.12.2] - 2022-06-04

### Improvement

- Add query_all method to support result pagination (Thanks to @LiHRaM).

## [0.12.1] - 2022-05-21

### Fix

- Fix dependencies issues (Thanks to @MathiasKindberg).
- Fix view creation (Thanks to @jeffreybolle).

## [0.12.0] - 2022-04-17

### Improvement

- Replace Chrome by Time.

## [0.11.0] - 2022-01-03

### Fix

- Fix QueryParameter type declaration (Thanks to @newapplesho).

## [0.10.2] - 2021-10-21

### Fix

- Fix 2 issues in the job API (Thanks to @nixxholas).

## [0.10.1] - 2021-10-31

### Added

- Add a BigQuery load job example in the examples directory.

## [0.10.0] - 2021-10-09

### Added

- 2 features `native-tls` and `rust-tls` to respectively use OpenSSL or Rust TLS.
- Add methods `is_empty`, `len`, `clear` to `TableDataInsertAllRequest`.
- Add method `add_rows` to `TableDataInsertAllRequest` (thanks @nixxholas).
- Bump version for yup-oauth2 v6 (thanks @JamesHinshelwood).
- Implement `Default` trait for most of the structures in the model sub-directory.
- Implement `Clone` trait for most of the structures in the model sub-directory.
- Implement `Display` trait for `ErrorProto` and `TableDataInsertAllResponseInsertErrors`.

## [0.9.3] - 2021-08-31

### Fix

- Fix ResultSet.get_i64 not working with some valid integer notation (e.g. 123.45E4) (Thanks to @komi1230).

## [0.9.2] - 2021-08-30

### Fix

- Fix Workload identify serialization issue (Thanks to @komi1230).

## [0.9.1] - 2021-08-16

### Added

- Workload identify support (Thanks to @komi1230).

## [0.9.0] - 2021-03-21

> Warning: this version is incompatible with the previous version.

### Added

- Added dataset.delete_if_exists.
- Added table.delete_if_exists.
- Added new setters for dataset and table structs.

### Changed

- Removed redundant parameters in dataset.create.
- Removed redundant paramaters in table.create.
- Improved BQError::ResponseError.
- Improved the example.
