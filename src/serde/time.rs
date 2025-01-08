use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use time::OffsetDateTime;

/// Deserialize an `OffsetDateTime` from its Unix timestamp with microseconds
pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<OffsetDateTime, D::Error> {
    let value = <f64>::deserialize(deserializer)?;
    let milliseconds = ((value - f64::floor(value)) * 1000.0) as i64;
    let timestamp = OffsetDateTime::from_unix_timestamp(value as i64).map_err(|e| D::Error::custom(e.to_string()))?;

    Ok(timestamp + time::Duration::milliseconds(milliseconds))
}

pub fn serialize<S: Serializer>(value: &OffsetDateTime, s: S) -> Result<S::Ok, S::Error> {
    let milliseconds = (value.millisecond() as f64) / 1000f64;
    let seconds = value.unix_timestamp() as f64;

    let value: f64 = seconds + milliseconds;

    value.serialize(s)
}

/// Treat an `Option<OffsetDateTime>` as a [Unix timestamp] with microseconds
/// for the purposes of serde.
///
/// Use this module in combination with serde's [`#[with]`][with] attribute.
///
/// When deserializing, the offset is assumed to be UTC.
///
/// [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time
/// [with]: https://serde.rs/field-attrs.html#with
pub mod option {

    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};
    use serde_crate::Serialize;
    use time::OffsetDateTime;

    /// Deserialize an `Option<OffsetDateTime>` from its Unix timestamp with microseconds
    pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error> {
        Option::deserialize(deserializer)?
            .map(|value: f64| {
                let milliseconds = ((value - f64::floor(value)) * 1000.0) as i64;
                let timestamp = OffsetDateTime::from_unix_timestamp(value as i64);
                timestamp.map(|t| t + time::Duration::milliseconds(milliseconds))
            })
            .transpose()
            .map_err(D::Error::custom)
    }

    pub fn serialize<S: Serializer>(value: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error> {
        value
            .map(|value| {
                let milliseconds = value.millisecond() as f64 / 1000f64;
                let seconds = value.unix_timestamp() as f64;
                seconds + milliseconds
            })
            .serialize(serializer)
    }
}

#[cfg(test)]
mod test {
    use crate::{model::table_field_schema::TableFieldSchema, serde::de::from_value};

    #[test]
    fn deserialize_time_column() {
        let test = serde_json::json!({"f": [{
            "v": "1.707366818084861E9"
        }]});

        #[derive(Deserialize)]
        pub struct A {
            #[serde(with = "crate::serde::time")]
            pub timestamp: time::OffsetDateTime,
        }

        let schema = TableFieldSchema {
            fields: Some(vec![TableFieldSchema {
                name: "timestamp".into(),
                policy_tags: Default::default(),
                r#type: crate::model::field_type::FieldType::Timestamp,
                ..Default::default()
            }]),
            ..Default::default()
        };
        let a = from_value::<A>(&schema, &test).unwrap();
        assert_eq!(a.timestamp, time::macros::datetime!(2024-02-08 04:33:38.084 UTC));
    }

    #[test]
    fn serialize_time_column() {
        #[derive(Serialize)]
        pub struct A {
            #[serde(with = "crate::serde::time")]
            pub timestamp: time::OffsetDateTime,
        }

        let a = A {
            timestamp: time::macros::datetime!(2024-12-01 01:00:00.45 +00),
        };

        assert_eq!(r#"{"timestamp":1733014800.45}"#, serde_json::to_string(&a).unwrap());
    }
}
