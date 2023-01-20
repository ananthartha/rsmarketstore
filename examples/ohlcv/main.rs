use candle::candle::Candle;
use marketstore::{stream, Agent, Error, QueryParams};
use tonic::transport::Uri;

mod candle;

#[tokio::main]

async fn main() -> Result<(), Error> {
    let agent = Agent::connect(Uri::from_static("http://localhost:5995").into()).await;

    agent
        .query(QueryParams {
            symbols: vec!["NIFTY 50".to_string()],
            timeframe: marketstore::MIN,
            attrgroup: "OHLCV".to_string(),
            ..Default::default()
        })
        .await?
        .into_iter()
        .for_each(|data_set| {
            // println!("{:#?}", data_set.start_index);
            // println!("{:#?}", data_set.lengths);
            let candles: Vec<Candle> = data_set.data.unwrap().try_into().unwrap();
            println!("{:#?}", candles)
        });

    let (stream, receiver) = stream::connect::<Candle>("endpoint").await.unwrap();
    Ok(())
}
