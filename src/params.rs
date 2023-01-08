use std::time::SystemTime;

use crate::{Interval, proto::MultiQueryRequest};

#[derive(Debug, Clone)]
pub struct QueryParams {
    symbols: Vec<String>,
    timeframe: Interval,
    attrgroup: String,
    start: Option<SystemTime>,
    end: Option<SystemTime>,
    limit: Option<u32>,
    limit_from_start: Option<bool>,
}

impl From<QueryParams> for MultiQueryRequest {
    fn from(value: QueryParams) -> Self {
        todo!()
    }
}