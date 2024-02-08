use serde::de::{self, DeserializeSeed, Error as SerdeError, SeqAccess, Visitor};
use serde_json::Value;

use base64::engine::Engine;

use crate::model::table_field_schema::TableFieldSchema;

use super::Error;

// Deserialize from a bigquery json value
pub fn from_value<'a, T>(schema: &'a TableFieldSchema, json: &'a Value) -> Result<T, Error>
where
    T: serde::Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_root_value(schema, json);
    T::deserialize(&mut deserializer)
}

pub fn from_column<'a, T>(schema: &'a TableFieldSchema, json: &'a Value) -> Result<T, Error>
where
    T: serde::Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_value(schema, json);
    T::deserialize(&mut deserializer)
}

pub struct Deserializer<'de> {
    schema: &'de TableFieldSchema,
    input: &'de Value,
    extractor: fn(&serde_json::Value) -> Option<&serde_json::Value>,
}

impl<'de> Deserializer<'de> {
    pub(crate) fn from_value(schema: &'de TableFieldSchema, input: &'de Value) -> Self {
        Deserializer {
            schema,
            input,
            extractor: extract,
        }
    }

    pub(crate) fn from_root_value(schema: &'de TableFieldSchema, input: &'de Value) -> Self {
        Deserializer {
            schema,
            input,
            extractor: root_extract,
        }
    }
}

macro_rules! bq_deserialize {
    ($method:ident, $t:ident, $visitor_func:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match extract(self.input) {
                Some(Value::String(b)) => {

                    let value = b
                        .parse::<$t>()
                        .map_err(|e| Error::Deserialization(e.to_string()))?;
                    visitor.$visitor_func(value)
                }
                a => {
                    Err(Error::custom(format!("unexpected value {} for {}", type_hint(a), self.schema.name)))
                }
            }
        }
    };
}

fn extract(value: &serde_json::Value) -> Option<&serde_json::Value> {
    match value {
        Value::Object(o) => o.get("v"),
        k => Some(k),
    }
}

fn root_extract(value: &serde_json::Value) -> Option<&serde_json::Value> {
    Some(value)
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    bq_deserialize!(deserialize_i8, i8, visit_i8);
    bq_deserialize!(deserialize_i16, i16, visit_i16);
    bq_deserialize!(deserialize_i32, i32, visit_i32);
    bq_deserialize!(deserialize_i64, i64, visit_i64);

    bq_deserialize!(deserialize_u8, u8, visit_u8);
    bq_deserialize!(deserialize_u16, u16, visit_u16);
    bq_deserialize!(deserialize_u32, u32, visit_u32);
    bq_deserialize!(deserialize_u64, u64, visit_u64);

    bq_deserialize!(deserialize_f32, f32, visit_f32);
    bq_deserialize!(deserialize_f64, f64, visit_f64);
    bq_deserialize!(deserialize_bool, bool, visit_bool);

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match extract(self.input) {
            Some(Value::String(b)) => visitor.visit_borrowed_str(b),
            a => Err(Error::custom(format!("unexpected value {} for {}", type_hint(a), self.schema.name))),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match extract(self.input) {
            Some(Value::String(b)) => {
                let bytes_buf = base64::engine::general_purpose::STANDARD
                    .decode(b)
                    .map_err(|e| Error::Deserialization(e.to_string()))?;
                visitor.visit_byte_buf(bytes_buf)
            }
            a => Err(Error::invalid_type(type_hint(a), &"JSON string")),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match (self.extractor)(self.input) {
            Some(o) if !matches!(o, Value::Null) => visitor.visit_some(self),
            _ => visitor.visit_none(),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match extract(self.input) {
            Some(Value::Null) => visitor.visit_unit(),
            a => Err(Error::invalid_type(type_hint(a), &"JSON NULL")),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match (self.extractor)(self.input) {
            Some(Value::Object(v)) => {
                let fields = v.get("f");
                match fields {
                    Some(Value::Array(input)) => {
                        let map = SeqDeserializer::new(self.schema, input);
                        visitor.visit_seq(map)
                    }
                    a => Err(Error::invalid_type(type_hint(a), &"JSON Array")),
                }
            }
            Some(Value::Array(fields)) => {
                let seq = SeqDeserializer::new(self.schema, fields);
                visitor.visit_seq(seq)
            }
            a => Err(Error::invalid_type(type_hint(a), &"JSON Array")),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match (self.extractor)(self.input) {
            Some(Value::Object(v)) => {
                let fields = v.get("f");
                match fields {
                    Some(Value::Array(input)) => {
                        let Some(schema) = self.schema.fields.as_ref() else {
                            return Err(Error::Deserialization("missing schema".into()));
                        };
                        let map = MapRefDeserializer::new(schema, input);
                        visitor.visit_map(map)
                    }
                    a => Err(Error::invalid_type(type_hint(a), &"JSON Array")),
                }
            }
            a => Err(Error::invalid_type(type_hint(a), &"JSON Object")),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

struct SeqDeserializer<'de> {
    schema: &'de TableFieldSchema,
    iter: <&'de Vec<Value> as IntoIterator>::IntoIter,
    value: Option<&'de Value>,
}

impl<'de> SeqDeserializer<'de> {
    fn new(schema: &'de TableFieldSchema, map: &'de [Value]) -> Self {
        SeqDeserializer {
            schema,
            iter: map.iter(),
            value: None,
        }
    }
}

impl<'de> SeqAccess<'de> for SeqDeserializer<'de> {
    type Error = Error;

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => {
                self.value = Some(value);
                let mut key = Deserializer::from_value(self.schema, value);
                seed.deserialize(&mut key).map(Some)
            }
            None => Ok(None),
        }
    }
}

struct MapRefDeserializer<'de> {
    schema: <&'de Vec<TableFieldSchema> as IntoIterator>::IntoIter,
    iter: <&'de Vec<Value> as IntoIterator>::IntoIter,
    schema_value: Option<&'de TableFieldSchema>,
}

impl<'de> MapRefDeserializer<'de> {
    fn new(schema: &'de [TableFieldSchema], values: &'de [Value]) -> Self {
        Self {
            schema: schema.iter(),
            iter: values.iter(),
            schema_value: None,
        }
    }
}

impl<'de> de::MapAccess<'de> for MapRefDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        let Some(key) = self.schema.next() else {
            return Ok(None);
        };
        let mut deserializer = crate::serde::de::table_row::MapKeyDeserializer { input: key };
        self.schema_value = Some(key);
        seed.deserialize(&mut deserializer).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let Some(value) = self.iter.next() else {
            return Err(Error::Deserialization("expected value but none".into()));
        };
        let Some(schema) = self.schema_value.take() else {
            return Err(Error::Deserialization("expected key but none".into()));
        };

        let mut deserializer = Deserializer::from_value(schema, value);
        seed.deserialize(&mut deserializer)
    }
}

fn type_hint(value: Option<&Value>) -> serde::de::Unexpected {
    match value {
        Some(Value::Null) => de::Unexpected::Other("null"),
        Some(Value::Bool(b)) => de::Unexpected::Bool(*b),
        Some(Value::Number(_)) => de::Unexpected::Other("number"),
        Some(Value::String(_)) => de::Unexpected::Other("string"),
        Some(Value::Array(_)) => de::Unexpected::Seq,
        Some(Value::Object(_)) => de::Unexpected::StructVariant,
        None => de::Unexpected::Other("missing datatype"),
    }
}

#[cfg(test)]
mod test {
    use crate::{
        model::{field_type::FieldType, table_field_schema::TableFieldSchema},
        serde::de::table_cell::from_column,
    };

    use super::from_value;

    #[test]
    fn bool_column() {
        let test = serde_json::json!({
            "v": "true"
        });

        let schema = TableFieldSchema {
            name: "mybool".into(),
            policy_tags: Default::default(),
            r#type: crate::model::field_type::FieldType::Bool,
            ..Default::default()
        };
        let b: Result<bool, _> = from_column(&schema, &test);
        assert!(b.is_ok());
        assert!(b.unwrap());
    }

    #[test]
    fn struct_column() {
        #[derive(Debug, serde::Deserialize)]
        pub struct A {
            mybool: bool,
        }
        let test = serde_json::json!({
            "v": {"f": [{"v": "true"}]}
        });

        let schema = TableFieldSchema {
            name: "a".into(),
            policy_tags: Default::default(),
            r#type: crate::model::field_type::FieldType::Struct,
            fields: Some(vec![TableFieldSchema {
                name: "mybool".into(),
                r#type: crate::model::field_type::FieldType::Bool,
                ..Default::default()
            }]),
            ..Default::default()
        };
        let b: Result<A, _> = from_column(&schema, &test);
        assert!(b.unwrap().mybool);
    }

    #[test]
    fn simple_struct() {
        #[derive(Debug, serde::Deserialize)]
        struct A {
            test: bool,
            number: i32,
        }

        let test = serde_json::json!({
            "f": [
                {
                    "v": "8"
                },
                {
                    "v": "Video Games"
                },
                {
                    "v": "true",
                }
            ]
        });
        let schema = TableFieldSchema {
            fields: Some(vec![
                TableFieldSchema {
                    name: "number".into(),
                    r#type: FieldType::Int64,
                    ..Default::default()
                },
                TableFieldSchema {
                    name: "ignored".into(),
                    r#type: FieldType::String,
                    ..Default::default()
                },
                TableFieldSchema {
                    name: "test".into(),
                    r#type: FieldType::Bool,
                    ..Default::default()
                },
            ]),
            name: "row".into(),
            r#type: crate::model::field_type::FieldType::Struct,
            ..Default::default()
        };

        let b: Result<A, _> = from_value(&schema, &test);

        assert!(b.is_ok(), "unexpected error {:?}", b);

        let b = b.unwrap();

        assert!(b.test);
        assert_eq!(b.number, 8);
    }

    #[test]
    fn tuple() {
        let a = serde_json::json!({
            "f": [
                { "v": "2" },
                { "v": "3" }
            ]
        });

        let schema = TableFieldSchema {
            fields: Some(vec![
                TableFieldSchema {
                    name: "_f0".into(),
                    r#type: FieldType::Int64,
                    ..Default::default()
                },
                TableFieldSchema {
                    name: "_fq".into(),
                    r#type: FieldType::Bool,
                    ..Default::default()
                },
            ]),
            name: "mybool".into(),
            r#type: crate::model::field_type::FieldType::Struct,
            ..Default::default()
        };
        let b: Result<(i64, i64), _> = from_value(&schema, &a);

        assert!(b.is_ok(), "failure: {:?}", b);

        let (a, b) = b.unwrap();
        assert_eq!(a, 2);
        assert_eq!(b, 3);
    }

    #[test]
    fn seq() {
        #[derive(Debug, serde::Deserialize)]
        struct A {
            numbers: Vec<i32>,
        }
        let test = serde_json::json!({
            "f": [
                { "v": [
                    { "v": "2" },
                    { "v": "3" }
                ]}
            ]
        });
        let schema = TableFieldSchema {
            name: "row".into(),
            fields: Some(vec![TableFieldSchema {
                name: "numbers".into(),
                r#type: FieldType::Int64,
                mode: Some("REPEATED".into()),
                ..Default::default()
            }]),
            ..Default::default()
        };
        let b: Result<A, _> = from_value(&schema, &test);

        assert!(b.is_ok(), "failure: {:?}", b);

        let b = b.unwrap();
        assert_eq!(b.numbers.len(), 2);
        assert_eq!(b.numbers.first().copied(), Some(2));
        assert_eq!(b.numbers.get(1).copied(), Some(3));
    }

    #[test]
    fn nested_seq() {
        #[derive(Debug, serde::Deserialize)]
        struct B {
            number: f64,
            string: String,
        }

        #[derive(Debug, serde::Deserialize)]
        pub struct A {
            pub b: Vec<B>,
        }

        let test = serde_json::json!({
            "f": [{
                "v": [{
                    "v": {
                        "f": [
                            { "v":"7.1" },
                            { "v": "Furniture" }
                        ]
                    }
                }]
            }]
        });
        let schema = TableFieldSchema {
            fields: Some(vec![TableFieldSchema {
                name: "b".into(),
                r#type: FieldType::Struct,
                mode: Some("REPEATED".into()),
                fields: Some(vec![
                    TableFieldSchema {
                        name: "number".into(),
                        r#type: FieldType::Float64,
                        ..Default::default()
                    },
                    TableFieldSchema {
                        name: "string".into(),
                        r#type: FieldType::String,
                        ..Default::default()
                    },
                ]),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let a: Result<A, _> = from_value(&schema, &test);

        assert!(a.is_ok(), "unexpected error: {:?}", a);

        let b = a.unwrap().b;
        assert_eq!(b.len(), 1);

        let b = b.first().unwrap();
        assert!(f64::abs(b.number - 7.1) < f64::EPSILON);
        assert_eq!(&b.string, "Furniture");
    }

    #[test]
    fn nested_struct_with_option() {
        #[derive(serde::Deserialize)]
        struct Outer {
            arr: Vec<(i64, Option<i64>)>,
            number: i64,
        }

        let schema = TableFieldSchema {
            fields: Some(vec![
                TableFieldSchema {
                    name: "arr".into(),
                    r#type: FieldType::Struct,
                    mode: Some("REPEATED".into()),
                    fields: Some(vec![
                        TableFieldSchema {
                            name: "number".into(),
                            r#type: FieldType::Float64,
                            ..Default::default()
                        },
                        TableFieldSchema {
                            name: "string".into(),
                            r#type: FieldType::String,
                            ..Default::default()
                        },
                    ]),
                    ..Default::default()
                },
                TableFieldSchema {
                    name: "number".into(),
                    r#type: FieldType::Int64,
                    mode: Some("NULLABLE".into()),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };

        let test = serde_json::json!({
            "f": [
              { "v": [ { "v": { "f": [{ "v": "2" }, { "v": null } ] } } ] },
              { "v": "2" },
            ]
        });

        let outer: Outer = from_value(&schema, &test).unwrap();
        assert_eq!(outer.arr.len(), 1);
        assert_eq!(outer.number, 2);
    }
}
