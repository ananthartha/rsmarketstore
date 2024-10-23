use crate::proto::{
  list_symbols_request::Format, ListSymbolsRequest, MultiQueryRequest, NumpyMultiDataset,
};

pub use crate::error::Error;
pub use crate::interval::*;
pub use crate::params::QueryParams;
pub use crate::proto::marketstore_client::MarketstoreClient;
pub use tonic::transport;

#[derive(Debug, Clone)]
pub struct Agent {
    client: MarketstoreClient<tonic::transport::Channel>,
}

impl Agent {
    pub async fn connect(dst: transport::Endpoint) -> Self {
        Self {
            client: MarketstoreClient::connect(dst).await.unwrap(),
        }
    }

    pub async fn query(self: &Self, params: QueryParams) -> crate::Result<Vec<NumpyMultiDataset>> {
        self.client
            .clone()
            .query(Into::<MultiQueryRequest>::into(params.clone()))
            .await
            .map(|response| {
                response
                    .into_inner()
                    .responses
                    .into_iter()
                    .flat_map(|query_response| query_response.result)
                    .collect::<Vec<_>>()
            })
            .map_err(|source| Error::QuerySymbolsError { params, source })
    }

    pub async fn list_symbols(self: &Self, format: Format) -> crate::Result<Vec<String>> {
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