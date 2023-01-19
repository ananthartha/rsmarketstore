use std::time::SystemTime;

use crate::{Interval, proto::MultiQueryRequest};

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

impl From<QueryParams> for MultiQueryRequest {
    fn from(value: QueryParams) -> Self {
        todo!()
    }
}