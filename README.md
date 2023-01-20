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
```

### Serde with Protobuf

Source can be found in `examples/ohlcv`

```bash
# Run
cargo run --example ohlcv --features=serde
```
