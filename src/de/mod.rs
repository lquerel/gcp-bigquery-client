use std::fmt;

use crate::model::{table_row::TableRow, table_schema::TableSchema};

pub mod table_cell;
mod table_row;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error while deserializing: {0}")]
    DeserializationError(String),
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::DeserializationError(msg.to_string())
    }
}

pub fn from_value<'a, T>(schema: &'a TableSchema, input: &'a TableRow) -> Result<T, Error>
where
    T: serde::Deserialize<'a>,
{
    let mut deserializer = table_row::TableRowDeserializer { schema, input };
    T::deserialize(&mut deserializer)
}
