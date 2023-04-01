#[derive(serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candle {
    #[prost(double, tag = "1")]
    #[serde(rename = "Open")]
    pub open: f64,
    #[prost(double, tag = "2")]
    #[serde(rename = "High")]
    pub high: f64,
    #[prost(double, tag = "3")]
    #[serde(rename = "Low")]
    pub low: f64,
    #[prost(double, tag = "4")]
    #[serde(rename = "Close")]
    pub close: f64,
    #[prost(double, tag = "5")]
    #[serde(rename = "Volume")]
    pub volume: f64,
    #[prost(int64, tag = "6")]
    #[serde(rename = "Epoch")]
    pub epoch: i64,
}
