# rsmarketstore
Rust driver for MarketStore

rsmarketstore can query and [write `TBD`] financial timeseries data from [MarketStore](https://github.com/alpacahq/marketstore)

## How to install

```bash
cargo add rsmarketstore
```

## Examples

```rust
// Connect
let agent = Agent::connect(
    Uri::from_static("http://localhost:5995").into()).await;

// Query
agent
    .query(QueryParams {
        symbols: vec!["NIFTY 50".to_string()],
        timeframe: marketstore::MIN,
        attrgroup: "OHLCV".to_string(),
        ..Default::default()
    })
    .await?

// timeframes
let FiveMins = 5 * marketstore::MIN;
let FifteenMin = 15 * marketstore::MIN;

let TwoHours = 2 * marketstore::HOUR;

// stream
    let (stream, receiver) = stream::connect::<Candle>("ws://localhost:5993/ws")
        .await
        .unwrap();

    stream.subscribe(vec!["NIFTY 50".into()]).await?;
    receiver
        .for_each(|msg| async move {
            println!("{:#?}", msg);
        })
        .await;
```

### Serde with Protobuf

Source can be found in `examples/ohlcv`

```bash
# Run
cargo run --example ohlcv --features=serde
```

### Stream

Source can be found in `examples/stream`

```bash
# Run
cargo run --example stream --features=stream
```
