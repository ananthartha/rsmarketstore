use std::time::SystemTime;

use crate::{
    proto::{MultiQueryRequest, QueryRequest},
    Interval,
};

#[derive(Debug, Clone)]
pub struct QueryParams {
    pub symbols: Vec<String>,
    pub timeframe: Interval,
    pub attrgroup: String,
    pub start: Option<SystemTime>,
    pub end: Option<SystemTime>,
    pub limit: Option<u32>,
    pub limit_from_start: Option<bool>,
}

impl Default for QueryParams {
    fn default() -> Self {
        Self {
            symbols: Default::default(),
            timeframe: Default::default(),
            attrgroup: Default::default(),
            start: Default::default(),
            end: Default::default(),
            limit: Default::default(),
            limit_from_start: Default::default(),
        }
    }
}

impl From<QueryParams> for MultiQueryRequest {
    fn from(value: QueryParams) -> Self {
        MultiQueryRequest {
            requests: value
                .symbols
                .into_iter()
                .map(|symbol| QueryRequest {
                    destination: format!(
                        "{}/{}/{}",
                        symbol,
                        String::from(value.timeframe),
                        value.attrgroup
                    ),
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
        }
    }
}
