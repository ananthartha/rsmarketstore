mod error;
mod serde;
mod value;
use self::error::Error;
pub use self::serde::from_dataset;
pub use value::Value;

#[derive(Clone, Copy)]
pub enum NumPyType {
    i1,
    i2,
    i4,
    i8,
    u1,
    u2,
    u4,
    u8,
    // f2,
    f4,
    f8,
    // f16,
}

impl TryFrom<String> for NumPyType {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "i1" => Ok(Self::i1),
            "i2" => Ok(Self::i2),
            "i4" => Ok(Self::i4),
            "i8" => Ok(Self::i8),
            "u1" => Ok(Self::u1),
            "u2" => Ok(Self::u4),
            "u4" => Ok(Self::u4),
            "u8" => Ok(Self::u8),
            // "f2" => Ok(Self::f2),
            "f4" => Ok(Self::f4),
            "f8" => Ok(Self::f8),
            // "f16" => Ok(Self::f16),
            _ => Err(Error::Unsupported {
                r#type: String::from(value),
            }),
        }
    }
}

impl From<NumPyType> for String {
    fn from(value: NumPyType) -> Self {
        match value {
            NumPyType::i1 => "i1",
            NumPyType::i2 => "i2",
            NumPyType::i4 => "i4",
            NumPyType::i8 => "i8",
            NumPyType::u1 => "u1",
            NumPyType::u2 => "u2",
            NumPyType::u4 => "u4",
            NumPyType::u8 => "u8",
            // DType::f2 => "f2",
            NumPyType::f4 => "f4",
            NumPyType::f8 => "f8",
            // DType::f16 => "f16",
        }
        .to_string()
    }
}

