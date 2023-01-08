#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataShape {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// DataType type = 2;
    /// type string such as i4 and f8
    /// use string instead of DataType enum in order to align with column_types in NumpyDataset.
    /// TODO: use DataType enum at DataShape and NumpyDataset
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumpyMultiDataset {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<NumpyDataset>,
    #[prost(map = "string, int32", tag = "2")]
    pub start_index: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    #[prost(map = "string, int32", tag = "3")]
    pub lengths: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumpyDataset {
    /// a list of type strings such as i4 and f8
    #[prost(string, repeated, tag = "1")]
    pub column_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// a list of column names
    #[prost(string, repeated, tag = "2")]
    pub column_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// two dimentional byte arrays holding the column data
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub column_data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, tag = "4")]
    pub length: i32,
    /// hidden
    #[prost(message, repeated, tag = "5")]
    pub data_shapes: ::prost::alloc::vec::Vec<DataShape>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRequest {
    /// a time bucket key
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub data_shapes: ::prost::alloc::vec::Vec<DataShape>,
    /// fixed or variable
    #[prost(string, tag = "3")]
    pub row_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiCreateRequest {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<CreateRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiQueryRequest {
    ///
    /// A multi-request allows for different Timeframes and record formats for each request
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<QueryRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    /// Note: SQL is not fully supported
    ///
    /// If this is a SQL request, Only SQLStatement is relevant
    #[prost(bool, tag = "1")]
    pub is_sql_statement: bool,
    #[prost(string, tag = "2")]
    pub sql_statement: ::prost::alloc::string::String,
    /// Destination is <symbol>/<timeframe>/<attributegroup>
    #[prost(string, tag = "3")]
    pub destination: ::prost::alloc::string::String,
    /// This is not usually set, defaults to Symbol/Timeframe/AttributeGroup
    #[prost(string, tag = "4")]
    pub key_category: ::prost::alloc::string::String,
    /// Lower time predicate (i.e. index >= start) in unix epoch second
    #[prost(int64, tag = "5")]
    pub epoch_start: i64,
    /// fractional part (nano second) of epoch_start
    #[prost(int64, tag = "6")]
    pub epoch_start_nanos: i64,
    /// Upper time predicate (i.e. index <= end) in unix epoch second
    #[prost(int64, tag = "7")]
    pub epoch_end: i64,
    /// fractional part (nano second) of epoch_end
    #[prost(int64, tag = "8")]
    pub epoch_end_nanos: i64,
    /// Number of max returned rows from lower/upper bound
    #[prost(int32, tag = "9")]
    pub limit_record_count: i32,
    /// Set to true if LimitRecordCount should be from the lower
    #[prost(bool, tag = "10")]
    pub limit_from_start: bool,
    /// Array of column names to be returned
    #[prost(string, repeated, tag = "11")]
    pub columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Support for functions is experimental and subject to change
    #[prost(string, repeated, tag = "12")]
    pub functions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiQueryResponse {
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<QueryResponse>,
    /// Server Version
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub timezone: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<NumpyMultiDataset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiWriteRequest {
    ///
    /// A multi-request allows for different Timeframes and record formats for each request
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<WriteRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequest {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<NumpyMultiDataset>,
    #[prost(bool, tag = "2")]
    pub is_variable_length: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiServerResponse {
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<ServerResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerResponse {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
    /// Server Version
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiKeyRequest {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<KeyRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyRequest {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSymbolsRequest {
    #[prost(enumeration = "list_symbols_request::Format", tag = "1")]
    pub format: i32,
}
/// Nested message and enum types in `ListSymbolsRequest`.
pub mod list_symbols_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Format {
        /// symbol names (e.g. ["AAPL", "AMZN", ....])
        Symbol = 0,
        /// {symbol/timeframe/attributeGroup} names (e.g. ["AAPL/1Min/TICK", "AAPL/1Sec/OHLCV", "Amazon/1D/Tick",...])
        TimeBucketKey = 1,
    }
    impl Format {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Format::Symbol => "SYMBOL",
                Format::TimeBucketKey => "TIME_BUCKET_KEY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SYMBOL" => Some(Self::Symbol),
                "TIME_BUCKET_KEY" => Some(Self::TimeBucketKey),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSymbolsResponse {
    #[prost(string, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerVersionRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerVersionResponse {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataType {
    ///
    /// NOTE: The ordering of this enum must match the File Format order
    ///
    /// We define our own types here instead of using the (excellent!) built-in Go type system for the primary reason
    /// that we are serializing data to files and so need to have a (very!) stable on-disk representation that matches
    /// the processing we do internally.
    Unknown = 0,
    Float32 = 1,
    Int32 = 2,
    Float64 = 3,
    Int64 = 4,
    Epoch = 5,
    Byte = 6,
    Bool = 7,
    None = 8,
    String = 9,
    Int16 = 10,
    Uint8 = 11,
    Uint16 = 12,
    Uint32 = 13,
    Uint64 = 14,
    String16 = 15,
}
impl DataType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataType::Unknown => "UNKNOWN",
            DataType::Float32 => "FLOAT32",
            DataType::Int32 => "INT32",
            DataType::Float64 => "FLOAT64",
            DataType::Int64 => "INT64",
            DataType::Epoch => "EPOCH",
            DataType::Byte => "BYTE",
            DataType::Bool => "BOOL",
            DataType::None => "NONE",
            DataType::String => "STRING",
            DataType::Int16 => "INT16",
            DataType::Uint8 => "UINT8",
            DataType::Uint16 => "UINT16",
            DataType::Uint32 => "UINT32",
            DataType::Uint64 => "UINT64",
            DataType::String16 => "STRING16",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "FLOAT32" => Some(Self::Float32),
            "INT32" => Some(Self::Int32),
            "FLOAT64" => Some(Self::Float64),
            "INT64" => Some(Self::Int64),
            "EPOCH" => Some(Self::Epoch),
            "BYTE" => Some(Self::Byte),
            "BOOL" => Some(Self::Bool),
            "NONE" => Some(Self::None),
            "STRING" => Some(Self::String),
            "INT16" => Some(Self::Int16),
            "UINT8" => Some(Self::Uint8),
            "UINT16" => Some(Self::Uint16),
            "UINT32" => Some(Self::Uint32),
            "UINT64" => Some(Self::Uint64),
            "STRING16" => Some(Self::String16),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod marketstore_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MarketstoreClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MarketstoreClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MarketstoreClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MarketstoreClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MarketstoreClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<super::MultiQueryRequest>,
        ) -> Result<tonic::Response<super::MultiQueryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Marketstore/Query");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::MultiCreateRequest>,
        ) -> Result<tonic::Response<super::MultiServerResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Marketstore/Create");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn write(
            &mut self,
            request: impl tonic::IntoRequest<super::MultiWriteRequest>,
        ) -> Result<tonic::Response<super::MultiServerResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Marketstore/Write");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn destroy(
            &mut self,
            request: impl tonic::IntoRequest<super::MultiKeyRequest>,
        ) -> Result<tonic::Response<super::MultiServerResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Marketstore/Destroy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_symbols(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSymbolsRequest>,
        ) -> Result<tonic::Response<super::ListSymbolsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Marketstore/ListSymbols",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn server_version(
            &mut self,
            request: impl tonic::IntoRequest<super::ServerVersionRequest>,
        ) -> Result<tonic::Response<super::ServerVersionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Marketstore/ServerVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
