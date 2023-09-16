use candle::candle::Candle;
use futures_util::StreamExt;
use marketstore::{stream, Error};
use tokio::runtime::Handle;

mod candle;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (stream, receiver) = stream::connect::<Candle>("ws://localhost:5993/ws")
        .await
        .unwrap();

    stream.subscribe(vec!["NIFTY 50/*/*".into(), "NIFTY2330217400CE/*/*".into(), "NIFTY2330217400PE/*/*".into()]).await?;
    receiver
        .for_each(|msg| async move {
            println!("{:#?}", msg);
        })
        .await;

    Ok(())
}
