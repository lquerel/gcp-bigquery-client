use serde::de;

use crate::model::table_field_schema::TableFieldSchema;

use super::Error;

macro_rules! unsupported {
    ($method:ident) => {
        fn $method<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            Err(Error::Deserialization(
                "non-string key is not supported".into(),
            ))
        }
    };
}

pub struct MapKeyDeserializer<'de> {
    pub input: &'de TableFieldSchema,
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut MapKeyDeserializer<'de> {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_borrowed_str(&self.input.name)
    }

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

    serde::forward_to_deserialize_any! {
        char str string bytes byte_buf unit unit_struct seq tuple tuple_struct
        map struct identifier ignored_any
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
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
        Err(Error::Deserialization("non-string unsupported".into()))
    }
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }
}
