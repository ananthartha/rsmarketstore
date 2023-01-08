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
}
