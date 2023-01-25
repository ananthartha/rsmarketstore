use thiserror::Error;

use crate::{proto::list_symbols_request::Format, QueryParams};

#[derive(Error, Debug)]
pub enum Error {
    #[doc(hidden)]
    #[error("Query Symbol {:#?}", params)]
    QuerySymbolsError {
        params: QueryParams,
        #[source]
        source: tonic::Status,
    },

    #[doc(hidden)]
    #[error("Unable to create consumer for {}", r#format.as_str_name())]
    ListSymbolsError {
        format: Format,
        #[source]
        source: tonic::Status,
    },

    #[cfg(feature="stream")]
    #[doc(hidden)]
    #[error("Error while encoding request {:#?}", request)]
    RequestEncodingError {
        request: crate::stream::Request,
        #[source]
        source: rmp_serde::encode::Error,
    },

    #[cfg(feature="stream")]
    #[doc(hidden)]
    #[error("Unable to create consumer for {:#?}", request)]
    UnableToSendRequestError {
        request: crate::stream::Request,
        #[source]
        source: futures_channel::mpsc::TrySendError<tokio_tungstenite::tungstenite::Message>,
    },
}
