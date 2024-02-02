use serde::{ser::Error, Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FieldType {
    #[default]
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
impl FieldType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FieldType::String => "STRING",
            FieldType::Bytes => "BYTES",
            FieldType::Integer => "INTEGER",
            FieldType::Int64 => "INT64",
            FieldType::Float => "FLOAT",
            FieldType::Float64 => "FLOAT64",
            FieldType::Numeric => "NUMERIC",
            FieldType::Bignumeric => "BIGNUMERIC",
            FieldType::Boolean => "BOOLEAN",
            FieldType::Bool => "BOOL",
            FieldType::Timestamp => "TIMESTAMP",
            FieldType::Date => "DATE",
            FieldType::Time => "TIME",
            FieldType::Datetime => "DATETIME",
            FieldType::Record => "RECORD",
            FieldType::Struct => "STRUCT",
            FieldType::Geography => "GEOGRAPHY",
            FieldType::Json => "JSON",
        }
    }
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self.as_str();
        write!(f, "{output}")
    }
}

pub fn serialize_json_as_string<S>(json: &serde_json::value::Value, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let string = serde_json::to_string(json).map_err(S::Error::custom)?;
    s.serialize_str(&string)
}
