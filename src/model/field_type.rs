use serde::{Deserialize, Serialize, Serializer};

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
    Geography,
    Json,
}

pub fn serialize_json_as_string<S>(json: &serde_json::value::Value, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let string = serde_json::to_string(json).unwrap();
    s.serialize_str(&string)
}