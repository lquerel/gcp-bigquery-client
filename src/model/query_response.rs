use crate::error::BQError;
use crate::model::error_proto::ErrorProto;
use crate::model::job_reference::JobReference;
use crate::model::table_row::TableRow;
use crate::model::table_schema::TableSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResponse {
    /// Whether the query result was fetched from the query cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_hit: Option<bool>,
    /// [Output-only] The first errors or warnings encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorProto>>,
    /// Whether the query has completed or not. If rows or totalRows are present, this will always be true. If this is false, totalRows will not be available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_complete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_reference: Option<JobReference>,
    /// The resource type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// [Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_dml_affected_rows: Option<String>,
    /// A token used for paging results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// An object with as many results as can be contained within the maximum permitted reply size. To get any additional rows, you can call GetQueryResults and specify the jobReference returned above.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<TableRow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<TableSchema>,
    /// The total number of bytes processed for this query. If this query was a dry run, this is the number of bytes that would be processed if the query were run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes_processed: Option<String>,
    /// The total number of rows in the complete query result set, which can be more than the number of rows in this single page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_rows: Option<String>,
}

/// Set of rows in response to a SQL query
#[derive(Debug)]
pub struct ResultSet {
    cursor: i64,
    row_count: i64,
    query_response: QueryResponse,
    fields: HashMap<String, usize>,
}

impl ResultSet {
    pub fn new(query_response: QueryResponse) -> Self {
        let row_count = query_response.rows.as_ref().map_or(0, |rows| rows.len()) as i64;
        let table_schema = query_response.schema.as_ref().expect("Expecting a schema");
        let table_fields = table_schema
            .fields
            .as_ref()
            .expect("Expecting a non empty list of fields");
        let fields: HashMap<String, usize> = table_fields
            .iter()
            .enumerate()
            .map(|(pos, field)| (field.name.clone(), pos))
            .collect();
        Self {
            cursor: -1,
            row_count,
            query_response,
            fields,
        }
    }

    pub fn query_response(&self) -> &QueryResponse {
        &self.query_response
    }

    /// Moves the cursor froward one row from its current position.
    /// A ResultSet cursor is initially positioned before the first row; the first call to the method next makes the
    /// first row the current row; the second call makes the second row the current row, and so on.
    pub fn next_row(&mut self) -> bool {
        if self.cursor == (self.row_count - 1) {
            false
        } else {
            self.cursor += 1;
            true
        }
    }

    /// Total number of rows in this result set.
    pub fn row_count(&self) -> usize {
        self.row_count as usize
    }

    /// List of column names for this result set.
    pub fn column_names(&self) -> Vec<String> {
        self.fields.keys().cloned().collect()
    }

    pub fn get_i64(&self, col_index: usize) -> Result<Option<i64>, BQError> {
        let json_value = self.get_json_value(col_index)?;
        match &json_value {
            None => Ok(None),
            Some(json_value) => match json_value {
                serde_json::Value::Number(value) => Ok(value.as_i64()),
                serde_json::Value::String(value) => {
                    let value: Result<i64, _> = value.parse();
                    match &value {
                        Err(_) => Err(BQError::InvalidColumnType {
                            col_index,
                            column_type: ResultSet::json_type(json_value),
                            type_requested: "I64".into(),
                        }),
                        Ok(value) => Ok(Some(*value)),
                    }
                }
                _ => Err(BQError::InvalidColumnType {
                    col_index,
                    column_type: ResultSet::json_type(&json_value),
                    type_requested: "I64".into(),
                }),
            },
        }
    }

    pub fn get_i64_by_name(&self, col_name: &str) -> Result<Option<i64>, BQError> {
        let col_index = self.fields.get(col_name);
        match col_index {
            None => Err(BQError::InvalidColumnName {
                col_name: col_name.into(),
            }),
            Some(col_index) => self.get_i64(*col_index),
        }
    }

    pub fn get_f64(&self, col_index: usize) -> Result<Option<f64>, BQError> {
        let json_value = self.get_json_value(col_index)?;
        match &json_value {
            None => Ok(None),
            Some(json_value) => match json_value {
                serde_json::Value::Number(value) => Ok(value.as_f64()),
                serde_json::Value::String(value) => {
                    let value: Result<f64, _> = value.parse();
                    match &value {
                        Err(_) => Err(BQError::InvalidColumnType {
                            col_index,
                            column_type: ResultSet::json_type(json_value),
                            type_requested: "F64".into(),
                        }),
                        Ok(value) => Ok(Some(*value)),
                    }
                }
                _ => Err(BQError::InvalidColumnType {
                    col_index,
                    column_type: ResultSet::json_type(&json_value),
                    type_requested: "F64".into(),
                }),
            },
        }
    }

    pub fn get_f64_by_name(&self, col_name: &str) -> Result<Option<f64>, BQError> {
        let col_index = self.fields.get(col_name);
        match col_index {
            None => Err(BQError::InvalidColumnName {
                col_name: col_name.into(),
            }),
            Some(col_index) => self.get_f64(*col_index),
        }
    }

    pub fn get_bool(&self, col_index: usize) -> Result<Option<bool>, BQError> {
        let json_value = self.get_json_value(col_index)?;
        match &json_value {
            None => Ok(None),
            Some(json_value) => match json_value {
                serde_json::Value::Bool(value) => Ok(Some(*value)),
                serde_json::Value::String(value) => {
                    let value: Result<bool, _> = value.parse();
                    match &value {
                        Err(_) => Err(BQError::InvalidColumnType {
                            col_index,
                            column_type: ResultSet::json_type(json_value),
                            type_requested: "Bool".into(),
                        }),
                        Ok(value) => Ok(Some(*value)),
                    }
                }
                _ => Err(BQError::InvalidColumnType {
                    col_index,
                    column_type: ResultSet::json_type(&json_value),
                    type_requested: "Bool".into(),
                }),
            },
        }
    }

    pub fn get_bool_by_name(&self, col_name: &str) -> Result<Option<bool>, BQError> {
        let col_index = self.fields.get(col_name);
        match col_index {
            None => Err(BQError::InvalidColumnName {
                col_name: col_name.into(),
            }),
            Some(col_index) => self.get_bool(*col_index),
        }
    }

    pub fn get_string(&self, col_index: usize) -> Result<Option<String>, BQError> {
        let json_value = self.get_json_value(col_index)?;
        match json_value {
            None => Ok(None),
            Some(json_value) => match json_value {
                serde_json::Value::String(value) => Ok(Some(value)),
                serde_json::Value::Number(value) => Ok(Some(value.to_string())),
                serde_json::Value::Bool(value) => Ok(Some(value.to_string())),
                _ => Err(BQError::InvalidColumnType {
                    col_index,
                    column_type: ResultSet::json_type(&json_value),
                    type_requested: "String".into(),
                }),
            },
        }
    }

    pub fn get_string_by_name(&self, col_name: &str) -> Result<Option<String>, BQError> {
        let col_index = self.fields.get(col_name);
        match col_index {
            None => Err(BQError::InvalidColumnName {
                col_name: col_name.into(),
            }),
            Some(col_index) => self.get_string(*col_index),
        }
    }

    pub fn get_json_value(&self, col_index: usize) -> Result<Option<serde_json::Value>, BQError> {
        if self.cursor < 0 || self.cursor == self.row_count {
            return Err(BQError::NoDataAvailable);
        }
        if col_index >= self.fields.len() {
            return Err(BQError::InvalidColumnIndex { col_index });
        }

        Ok(self
            .query_response
            .rows
            .as_ref()
            .and_then(|rows| rows.get(self.cursor as usize))
            .and_then(|row| row.columns.as_ref())
            .and_then(|cols| cols.get(col_index))
            .and_then(|col| col.value.clone()))
    }

    pub fn get_json_value_by_name(&self, col_name: &str) -> Result<Option<serde_json::Value>, BQError> {
        let col_pos = self.fields.get(col_name);
        match col_pos {
            None => Err(BQError::InvalidColumnName {
                col_name: col_name.into(),
            }),
            Some(col_pos) => self.get_json_value(*col_pos),
        }
    }

    fn json_type(json_value: &serde_json::Value) -> String {
        match json_value {
            Value::Null => "Null".into(),
            Value::Bool(_) => "Bool".into(),
            Value::Number(_) => "Number".into(),
            Value::String(_) => "String".into(),
            Value::Array(_) => "Array".into(),
            Value::Object(_) => "Object".into(),
        }
    }
}
