# Changelog

All notable changes to this project will be documented in this file.

## [0.10.0] - 2021-10-09

### Added

- Add method `add_rows` to `TableDataInsertAllRequest` (thanks @nixxholas)
- Bump version for yup-oauth2 v6 (thanks @JamesHinshelwood)
- Implement the Default trait for most of the structures in the model sub-directory
- Implement the Clone trait for most of the structures in the model sub-directory

## [0.9.3] - 2021-08-31

### Fix 

- Fix ResultSet.get_i64 not working with some valid integer notation (e.g. 123.45E4) (Thanks to @komi1230)


## [0.9.2] - 2021-08-30

### Fix

- Fix Workload identify serialization issue (Thanks to @komi1230)

## [0.9.1] - 2021-08-16

### Added

- Workload identify support (Thanks to @komi1230)

## [0.9.0] - 2021-03-21

> Warning: this version is incompatible with the previous version

### Added

- Added dataset.delete_if_exists.
- Added table.delete_if_exists.
- Added new setters for dataset and table structs.

### Changed

- Removed redundant parameters in dataset.create.
- Removed redundant paramaters in table.create.
- Improved BQError::ResponseError.
- Improved the example.
