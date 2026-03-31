<p align="center">
  <img src="https://www.kaiko.com/assets/sites/1/2025/10/kaiko-logo-rgb_color.svg" alt="Kaiko" width="300">
</p>

# kaiko-rust-sdk

Rust client library for the Kaiko gRPC API. This SDK provides auto-generated gRPC stubs for streaming cryptocurrency market data.

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
kaikosdk = { git = "https://github.com/kaikodata/kaiko-rust-sdk", tag = "1.34.0" }
```

## Requirements

- Rust 2024 edition
- A valid Kaiko API key

## Usage

```rust
use kaikosdk::sdk_client::StreamTradesServiceV1Client;
use kaikosdk::stream::trades_v1::StreamTradesRequestV1;
use kaikosdk::core::InstrumentCriteria;
use tonic::transport::Channel;
use tonic::metadata::MetadataValue;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("https://gateway-v0-grpc.kaiko.ovh:443")
        .tls_config(tonic::transport::ClientTlsConfig::new())?
        .connect()
        .await?;

    let token: MetadataValue<_> = "Bearer <YOUR_API_KEY>".parse()?;
    let mut client = StreamTradesServiceV1Client::with_interceptor(channel, move |mut req: tonic::Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let request = StreamTradesRequestV1 {
        instrument_criteria: Some(InstrumentCriteria {
            exchange: "cbse".into(),
            instrument_class: "spot".into(),
            code: "btc-usd".into(),
        }),
    };

    let mut stream = client.subscribe(request).await?.into_inner();

    while let Some(response) = stream.message().await? {
        println!("Trade: {:?}", response);
    }

    Ok(())
}
```

## Available Services

All services expose a `subscribe` method that returns a server-side streaming RPC.

| Service | Description |
|---------|-------------|
| `StreamTradesServiceV1` | Real-time trades |
| `StreamMarketUpdateServiceV1` | Market updates |
| `StreamAggregatedQuoteServiceV2` | Aggregated quotes |
| `StreamAggregatedPriceServiceV1` | Aggregated prices |
| `StreamAggregatedStatePriceServiceV1` | Aggregated state prices |
| `StreamAggregatesOHLCVServiceV1` | OHLCV aggregates |
| `StreamAggregatesVWAPServiceV1` | VWAP aggregates |
| `StreamAggregatesSpotExchangeRateV2ServiceV1` | Spot exchange rates |
| `StreamAggregatesSpotDirectExchangeRateV2ServiceV1` | Direct exchange rates |
| `StreamOrderbookL2ServiceV1` | Level 2 order book |
| `StreamOrderbookL2ReplayServiceV1` | Level 2 order book replay |
| `StreamIndexServiceV1` | Index values |
| `StreamIndexMultiAssetsServiceV1` | Multi-asset index values |
| `StreamIndexForexRateServiceV1` | Index forex rates |
| `StreamCompositeIndicesServiceV1` | Composite indices |
| `StreamConstantDurationIndicesServiceV1` | Constant duration indices |
| `StreamExoticIndicesServiceV1` | Exotic indices |
| `StreamDerivativesInstrumentMetricsServiceV1` | Derivatives instrument metrics |
| `StreamIvSviParametersServiceV1` | IV SVI parameters |

## Examples

For more complete examples, see the [kaiko-sdk-examples](https://github.com/kaikodata/kaiko-sdk-examples) repository.
