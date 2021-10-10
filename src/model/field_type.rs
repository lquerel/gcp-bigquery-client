use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FieldType {
    String,
    Bytes,
    Integer,
    Int64, // same as INTEGER
    Float,
    Float64, // same as FLOAT
    Numeric,
    Bignumeric,
    Boolean,
    Bool, // same as BOOLEAN
    Timestamp,
    Date,
    Time,
    Datetime,
    Record, // where RECORD indicates that the field contains a nested schema
    Struct, // same as RECORD
}
