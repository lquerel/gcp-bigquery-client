use serde::de::Error;
use serde::{Deserialize, Deserializer};

use time::OffsetDateTime;

pub use time::serde::rfc3339::serialize;

/// Deserialize an `OffsetDateTime` from its Unix timestamp with microseconds
pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<OffsetDateTime, D::Error> {
    let value = <f64>::deserialize(deserializer)?;
    let milliseconds = ((value - f64::floor(value)) * 1000.0) as i64;
    let timestamp = OffsetDateTime::from_unix_timestamp(value as i64).map_err(|e| D::Error::custom(e.to_string()))?;

    Ok(timestamp + time::Duration::milliseconds(milliseconds))
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
    use serde::{Deserialize, Deserializer};
    use time::OffsetDateTime;

    pub use time::serde::rfc3339::option::serialize;

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
}

#[cfg(test)]
mod test {
    use crate::{model::table_field_schema::TableFieldSchema, serde::de::from_value};

    #[test]
    fn time_column() {
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
}
