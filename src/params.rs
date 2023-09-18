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

fn system_time_as_epoch(system_time: std::time::SystemTime) -> (i64, i64) {
    let (seconds, nanos) = match system_time.duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => {
            let seconds = i64::try_from(duration.as_secs()).unwrap();
            (seconds, duration.subsec_nanos() as i64)
        }
        Err(error) => {
            let duration = error.duration();
            let seconds = i64::try_from(duration.as_secs()).unwrap();
            let nanos = duration.subsec_nanos() as i64;
            if nanos == 0 {
                (-seconds, 0)
            } else {
                (-seconds - 1, 1_000_000_000 - nanos)
            }
        }
    };
    return (seconds, nanos);
}

impl From<QueryParams> for MultiQueryRequest {
    fn from(value: QueryParams) -> Self {
        let (epoch_start, epoch_start_nanos) =
            value.start.map(system_time_as_epoch).unwrap_or_default();
        let (epoch_end, epoch_end_nanos) =
            value.start.map(system_time_as_epoch).unwrap_or_default();

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
                    epoch_start,
                    epoch_start_nanos,
                    epoch_end,
                    epoch_end_nanos,
                    limit_record_count: value.limit.unwrap_or_default() as i32,
                    limit_from_start: value.limit_from_start.unwrap_or_default(),
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
        }
    }
}
