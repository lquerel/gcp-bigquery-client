# Changelog

All notable changes to this project will be documented in this file.

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
