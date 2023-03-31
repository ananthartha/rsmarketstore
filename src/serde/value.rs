use serde::{
    de::{self, IntoDeserializer},
    forward_to_deserialize_any,
};

use super::error::{Error, Result};

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

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
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
        bool char str string
        bytes byte_buf unit unit_struct struct newtype_struct seq tuple
        tuple_struct map identifier ignored_any enum
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_none()
    }

    // i8 i16 i32 i128 u8 u16 u32 u64 u128 f32
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::I1(value) => visitor.visit_i8(value),
            Value::U1(value) => visitor.visit_i8(value as i8),
            _ => Err(Error::UnableToCast {
                r#type: String::from("i8"),
            }),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::I1(value) => visitor.visit_u8(value as u8),
            Value::U1(value) => visitor.visit_u8(value),
            _ => Err(Error::UnableToCast {
                r#type: String::from("u8"),
            }),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::I1(value) => visitor.visit_i16(value as i16),
            Value::I2(value) => visitor.visit_i16(value),
            Value::U1(value) => visitor.visit_i16(value as i16),
            Value::U2(value) => visitor.visit_i16(value as i16),
            _ => Err(Error::UnableToCast {
                r#type: String::from("i16"),
            }),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::I1(value) => visitor.visit_u16(value as u16),
            Value::I2(value) => visitor.visit_u16(value as u16),
            Value::U1(value) => visitor.visit_u16(value as u16),
            Value::U2(value) => visitor.visit_u16(value),
            _ => Err(Error::UnableToCast {
                r#type: String::from("u16"),
            }),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::I1(value) => visitor.visit_i32(value as i32),
            Value::I2(value) => visitor.visit_i32(value as i32),
            Value::I4(value) => visitor.visit_i32(value),
            Value::U1(value) => visitor.visit_i32(value as i32),
            Value::U2(value) => visitor.visit_i32(value as i32),
            Value::U4(value) => visitor.visit_i32(value as i32),
            Value::U8(value) => visitor.visit_i32(value as i32),
            Value::F2(value) => visitor.visit_i32(value as i32),
            Value::F4(value) => visitor.visit_i32(value as i32),
            _ => Err(Error::UnableToCast {
                r#type: String::from("i32"),
            }),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::I1(value) => visitor.visit_u32(value as u32),
            Value::I2(value) => visitor.visit_u32(value as u32),
            Value::I4(value) => visitor.visit_u32(value as u32),
            Value::U1(value) => visitor.visit_u32(value as u32),
            Value::U2(value) => visitor.visit_u32(value as u32),
            Value::U4(value) => visitor.visit_u32(value),
            Value::U8(value) => visitor.visit_u32(value as u32),
            Value::F2(value) => visitor.visit_u32(value as u32),
            Value::F4(value) => visitor.visit_u32(value as u32),
            _ => Err(Error::UnableToCast {
                r#type: String::from("u32"),
            }),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::I1(value) => visitor.visit_f32(value as f32),
            Value::I2(value) => visitor.visit_f32(value as f32),
            Value::I4(value) => visitor.visit_f32(value as f32),
            Value::U1(value) => visitor.visit_f32(value as f32),
            Value::U2(value) => visitor.visit_f32(value as f32),
            Value::U4(value) => visitor.visit_f32(value as f32),
            Value::U8(value) => visitor.visit_f32(value as f32),
            Value::F2(value) => visitor.visit_f32(value),
            Value::F4(value) => visitor.visit_f32(value),
            _ => Err(Error::UnableToCast {
                r#type: String::from("f32"),
            }),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::I1(value) => visitor.visit_i64(value as i64),
            Value::I2(value) => visitor.visit_i64(value as i64),
            Value::I4(value) => visitor.visit_i64(value as i64),
            Value::I8(value) => visitor.visit_i64(value),
            Value::U1(value) => visitor.visit_i64(value as i64),
            Value::U2(value) => visitor.visit_i64(value as i64),
            Value::U4(value) => visitor.visit_i64(value as i64),
            Value::U8(value) => visitor.visit_i64(value as i64),
            Value::F2(value) => visitor.visit_i64(value as i64),
            Value::F4(value) => visitor.visit_i64(value as i64),
            Value::F8(value) => visitor.visit_i64(value as i64),
            _ => Err(Error::UnableToCast {
                r#type: String::from("i64"),
            }),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::I1(value) => visitor.visit_u64(value as u64),
            Value::I2(value) => visitor.visit_u64(value as u64),
            Value::I4(value) => visitor.visit_u64(value as u64),
            Value::I8(value) => visitor.visit_u64(value as u64),
            Value::U1(value) => visitor.visit_u64(value as u64),
            Value::U2(value) => visitor.visit_u64(value as u64),
            Value::U4(value) => visitor.visit_u64(value as u64),
            Value::U8(value) => visitor.visit_u64(value),
            Value::F2(value) => visitor.visit_u64(value as u64),
            Value::F4(value) => visitor.visit_u64(value as u64),
            Value::F8(value) => visitor.visit_u64(value as u64),
            _ => Err(Error::UnableToCast {
                r#type: String::from("u64"),
            }),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::I1(value) => visitor.visit_f64(value as f64),
            Value::I2(value) => visitor.visit_f64(value as f64),
            Value::I4(value) => visitor.visit_f64(value as f64),
            Value::I8(value) => visitor.visit_f64(value as f64),
            Value::U1(value) => visitor.visit_f64(value as f64),
            Value::U2(value) => visitor.visit_f64(value as f64),
            Value::U4(value) => visitor.visit_f64(value as f64),
            Value::U8(value) => visitor.visit_f64(value as f64),
            Value::F2(value) => visitor.visit_f64(value as f64),
            Value::F4(value) => visitor.visit_f64(value as f64),
            Value::F8(value) => visitor.visit_f64(value),
            _ => Err(Error::UnableToCast {
                r#type: String::from("f64"),
            }),
        }
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::I1(value) => visitor.visit_i128(value as i128),
            Value::I2(value) => visitor.visit_i128(value as i128),
            Value::I4(value) => visitor.visit_i128(value as i128),
            Value::I8(value) => visitor.visit_i128(value as i128),
            Value::U1(value) => visitor.visit_i128(value as i128),
            Value::U2(value) => visitor.visit_i128(value as i128),
            Value::U4(value) => visitor.visit_i128(value as i128),
            Value::U8(value) => visitor.visit_i128(value as i128),
            Value::F2(value) => visitor.visit_i128(value as i128),
            Value::F4(value) => visitor.visit_i128(value as i128),
            Value::F8(value) => visitor.visit_i128(value as i128),
            _ => Err(Error::UnableToCast {
                r#type: String::from("i128"),
            }),
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::I1(value) => visitor.visit_u128(value as u128),
            Value::I2(value) => visitor.visit_u128(value as u128),
            Value::I4(value) => visitor.visit_u128(value as u128),
            Value::I8(value) => visitor.visit_u128(value as u128),
            Value::U1(value) => visitor.visit_u128(value as u128),
            Value::U2(value) => visitor.visit_u128(value as u128),
            Value::U4(value) => visitor.visit_u128(value as u128),
            Value::U8(value) => visitor.visit_u128(value as u128),
            Value::F2(value) => visitor.visit_u128(value as u128),
            Value::F4(value) => visitor.visit_u128(value as u128),
            Value::F8(value) => visitor.visit_u128(value as u128),
            _ => Err(Error::UnableToCast {
                r#type: String::from("u128"),
            }),
        }
    }
}

impl<'de> IntoDeserializer<'de, Error> for Value {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}
