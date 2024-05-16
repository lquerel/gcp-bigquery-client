use std::fmt;

pub mod table_cell;
mod table_row;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error while deserializing: {0}")]
    Deserialization(String),
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Deserialization(msg.to_string())
    }
}

pub use table_cell::from_value;

use crate::model::{
    get_query_results_response::GetQueryResultsResponse, table_row::TableRow, table_schema::TableSchema,
};

pub fn from_rows<T>(schema: &TableSchema, rows: &[TableRow]) -> Result<Vec<T>, Error>
where
    T: serde::de::DeserializeOwned,
{
    rows.iter()
        .map(|row| {
            let mut map = serde_json::Map::new();
            if let Some(array) = row.columns.as_ref().map(|c| {
                c.iter()
                    .map(|c| c.value.to_owned())
                    .map(|v| {
                        let mut map = serde_json::Map::new();
                        map.insert("v".into(), v.unwrap_or(serde_json::Value::Null));
                        map
                    })
                    .collect()
            }) {
                map.insert("f".into(), array);
            }

            let obj = serde_json::Value::Object(map);

            from_value(&schema.as_table_field_schema(), &obj)
        })
        .collect::<Result<Vec<T>, _>>()
}
pub fn from_query_results<T>(value: &GetQueryResultsResponse) -> Result<Vec<T>, Error>
where
    T: serde::de::DeserializeOwned,
{
    let schema = value.schema.as_ref().expect("schema must exist for completed job");

    let rows = value.rows.as_deref().unwrap_or_default();

    from_rows(schema, rows)
}
