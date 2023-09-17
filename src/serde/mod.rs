mod error;
mod serde;
mod value;
pub use self::error::Error;
pub use self::serde::from_dataset;
pub use value::Value;

#[derive(Clone, Copy)]
pub enum NumPyType {
    I1,
    I2,
    I4,
    I8,
    U1,
    U2,
    U4,
    U8,
    // f2,
    F4,
    F8,
    // f16,
}

impl TryFrom<String> for NumPyType {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "i1" => Ok(Self::I1),
            "i2" => Ok(Self::I2),
            "i4" => Ok(Self::I4),
            "i8" => Ok(Self::I8),
            "u1" => Ok(Self::U1),
            "u2" => Ok(Self::U4),
            "u4" => Ok(Self::U4),
            "u8" => Ok(Self::U8),
            // "f2" => Ok(Self::f2),
            "f4" => Ok(Self::F4),
            "f8" => Ok(Self::F8),
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
            NumPyType::I1 => "i1",
            NumPyType::I2 => "i2",
            NumPyType::I4 => "i4",
            NumPyType::I8 => "i8",
            NumPyType::U1 => "u1",
            NumPyType::U2 => "u2",
            NumPyType::U4 => "u4",
            NumPyType::U8 => "u8",
            // DType::f2 => "f2",
            NumPyType::F4 => "f4",
            NumPyType::F8 => "f8",
            // DType::f16 => "f16",
        }
        .to_string()
    }
}

