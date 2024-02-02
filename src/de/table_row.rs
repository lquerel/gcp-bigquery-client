use serde::de;

use crate::model::{
    table_cell::TableCell, table_field_schema::TableFieldSchema, table_row::TableRow, table_schema::TableSchema,
};

use super::Error;

pub struct TableRowDeserializer<'de> {
    pub schema: &'de TableSchema,
    pub input: &'de TableRow,
}

macro_rules! unsupported {
    ($method:ident) => {
        fn $method<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            Err(Error::DeserializationError(
                "row deserialization into base units is not supported".into(),
            ))
        }
    };
}
impl<'de, 'a> de::Deserializer<'de> for &'a mut TableRowDeserializer<'de> {
    type Error = Error;

    unsupported!(deserialize_any);
    unsupported!(deserialize_bool);
    unsupported!(deserialize_i8);
    unsupported!(deserialize_i16);
    unsupported!(deserialize_i32);
    unsupported!(deserialize_i64);

    unsupported!(deserialize_u8);
    unsupported!(deserialize_u16);
    unsupported!(deserialize_u32);
    unsupported!(deserialize_u64);

    unsupported!(deserialize_f32);
    unsupported!(deserialize_f64);
    unsupported!(deserialize_char);
    unsupported!(deserialize_str);
    unsupported!(deserialize_string);
    unsupported!(deserialize_bytes);
    unsupported!(deserialize_byte_buf);
    unsupported!(deserialize_option);

    unsupported!(deserialize_unit);

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeserializationError("unsupported sequence".into()))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let columns = self
            .input
            .columns
            .as_ref()
            .ok_or_else(|| Error::DeserializationError("no data".into()))?;

        let fields = self
            .schema
            .fields
            .as_ref()
            .ok_or_else(|| Error::DeserializationError("no schema".into()))?;

        let serializer = MapRefDeserializer::new(fields, columns);
        visitor.visit_map(serializer)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
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
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }
}

struct MapRefDeserializer<'de> {
    schema: <&'de Vec<TableFieldSchema> as IntoIterator>::IntoIter,
    iter: <&'de Vec<TableCell> as IntoIterator>::IntoIter,
    value: Option<&'de TableFieldSchema>,
}

impl<'de> MapRefDeserializer<'de> {
    fn new(schema: &'de [TableFieldSchema], values: &'de [TableCell]) -> Self {
        Self {
            schema: schema.iter(),
            iter: values.iter(),
            value: None,
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
        let mut deserializer = MapKeyDeserializer { input: key };
        self.value = Some(key);
        seed.deserialize(&mut deserializer).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let Some(value) = self.iter.next() else {
            return Err(Error::DeserializationError("expected value but none".into()));
        };
        let Some(schema) = self.value.take() else {
            return Err(Error::DeserializationError("expected key but none".into()));
        };

        let mut deserializer =
            crate::de::table_cell::Deserializer::from_value(schema, crate::de::table_cell::Input::Root(value));
        seed.deserialize(&mut deserializer)
    }
}

pub struct MapKeyDeserializer<'de> {
    pub input: &'de TableFieldSchema,
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut MapKeyDeserializer<'de> {
    type Error = Error;
    unsupported!(deserialize_any);
    unsupported!(deserialize_bool);
    unsupported!(deserialize_i8);
    unsupported!(deserialize_i16);
    unsupported!(deserialize_i32);
    unsupported!(deserialize_i64);

    unsupported!(deserialize_u8);
    unsupported!(deserialize_u16);
    unsupported!(deserialize_u32);
    unsupported!(deserialize_u64);

    unsupported!(deserialize_f32);
    unsupported!(deserialize_f64);
    unsupported!(deserialize_char);
    unsupported!(deserialize_str);
    unsupported!(deserialize_string);
    unsupported!(deserialize_bytes);
    unsupported!(deserialize_byte_buf);
    unsupported!(deserialize_option);

    unsupported!(deserialize_unit);

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeserializationError("non-string unsupported".into()))
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeserializationError("non-string unsupported".into()))
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeserializationError("non-string unsupported".into()))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeserializationError("non-string unsupported".into()))
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeserializationError("non-string unsupported".into()))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeserializationError("non-string unsupported".into()))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeserializationError("non-string unsupported".into()))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeserializationError("non-string unsupported".into()))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeserializationError("non-string unsupported".into()))
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeserializationError("non-string unsupported".into()))
    }
}
