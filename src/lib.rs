mod error;
mod interval;
mod params;
mod proto;
use proto::{list_symbols_request::Format, ListSymbolsRequest, MultiQueryRequest};
use std::time::SystemTime;
use tonic::transport::Endpoint;

pub use error::Error;
pub use interval::*;
pub use params::QueryParams;
pub use proto::marketstore_client::MarketstoreClient;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Agent {
    client: MarketstoreClient<tonic::transport::Channel>,
}

impl Agent {
    pub async fn connect(dst: Endpoint) -> Self {
        Self {
            client: MarketstoreClient::connect(dst).await.unwrap(),
        }
    }

    pub async fn query(self: &Self, params: QueryParams) -> Result<()> {
        self.client
            .clone()
            .query(Into::<MultiQueryRequest>::into(params.clone()))
            .await
            .map(|response| {
                response.into_inner().responses;
            })
            .map_err(|source| Error::QuerySymbolsError { params, source })
    }

    pub async fn list_symbols(self: &Self, format: Format) -> Result<Vec<String>> {
        self.client
            .clone()
            .list_symbols(ListSymbolsRequest {
                format: format.into(),
            })
            .await
            .map(|response| response.into_inner().results)
            .map_err(|source| Error::ListSymbolsError { format, source })
    }
}

impl From<MarketstoreClient<tonic::transport::Channel>> for Agent {
    fn from(client: MarketstoreClient<tonic::transport::Channel>) -> Self {
        Self { client }
    }
}
