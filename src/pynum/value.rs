use serde::{
    de::{self, IntoDeserializer},
    forward_to_deserialize_any,
};

use super::error::Error;

pub enum Value {
    I1(i8),
    I2(i16),
    I4(i32),
    I8(i64),
    U1(u8),
    U2(u16),
    U4(u32),
    U8(u64),
    F2(f32),
    F4(f32),
    F8(f64),
    // F16,
    Bool(bool),
    String(String),
}

impl<'de, 'a> de::Deserializer<'de> for Value {
    type Error = super::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::I1(value) => visitor.visit_i8(value),
            Value::I2(value) => visitor.visit_i16(value),
            Value::I4(value) => visitor.visit_i32(value),
            Value::I8(value) => visitor.visit_i64(value),
            Value::U1(value) => visitor.visit_u8(value),
            Value::U2(value) => visitor.visit_u16(value),
            Value::U4(value) => visitor.visit_u32(value),
            Value::U8(value) => visitor.visit_u64(value),
            Value::F2(value) => visitor.visit_f32(value),
            Value::F4(value) => visitor.visit_f32(value),
            Value::F8(value) => visitor.visit_f64(value),
            Value::Bool(value) => visitor.visit_bool(value),
            Value::String(value) => visitor.visit_string(value.to_string()),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct struct newtype_struct seq tuple
        tuple_struct map identifier ignored_any enum
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_none()
    }
}

impl<'de> IntoDeserializer<'de, Error> for Value {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}
