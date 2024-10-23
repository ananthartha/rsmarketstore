mod error;
pub mod interval;
mod params;
pub mod proto;
#[cfg(feature = "serde")]
pub mod serde;
#[cfg(feature = "stream")]
pub mod stream;
mod grpc;

pub use error::Error;
pub use interval::*;
pub use params::QueryParams;
pub use proto::marketstore_client::MarketstoreClient;
pub use tonic::transport;

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(not(target_arch = "wasm32"))]
pub use grpc::*;

pub use prost;