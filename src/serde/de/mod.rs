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
