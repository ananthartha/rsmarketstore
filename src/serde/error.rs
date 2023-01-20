use std::fmt::{self, Display};

use serde::{de, ser};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[doc(hidden)]
    #[error("{} column not found", column)]
    ColumnNotFound { column: String },

    #[doc(hidden)]
    #[error("unable to seed the column {} name as key for serda", column)]
    UnableSeedColumnKey { column: String },

    #[doc(hidden)]
    #[error("Serde Error {}", msg)]
    Serde { msg: String },

    #[doc(hidden)]
    #[error("Unsupport data type {r#type}")]
    Unsupported { r#type: String },
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Serde {
            msg: msg.to_string(),
        }
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Serde {
            msg: msg.to_string(),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
