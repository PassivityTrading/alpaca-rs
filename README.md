# alpaca-sdk, an unofficial Rust SDK for the Alpaca API (WIP)

## Features

- Ease of use - Minimal setup and no boilerplate.
- Cross-platform - can run on most platforms that Rust can run on - x86, aarch64, wasm (any platform that [`reqwest`](https://crates.io/crates/reqwest) can run on).
- Speed - uses [`reqwest`](https://crates.io/crates/reqwest) to make API requests, for performance.
- Interoperable - does not depend on any async runtime. Anything that can poll a future, including [`tokio`](https://crates.io/crates/tokio) and [`async-std`](https://crates.io/crates/async-std) will work.
- Supports **all** Alpaca APIs - Broker API, Trading API and the Market Data API.

## Guide

### Installing

#### Crates.io (currently not available)

Use this command:
```sh
cargo add alpaca-rs
```

Or add this to Cargo.toml:
```toml
[dependencies]
alpaca-rs = "<latest crates.io version>"
```

#### Latest commit (git)

Use this command:
```sh
cargo add alpaca-rs --git https://github.com/PassivityTrading/alpaca-rs
```

Or add this to Cargo.toml:
```toml
[dependencies]
# ... other dependencies ...
alpaca-rs.git = "https://github.com/PassivityTrading/alpaca-rs"
```

Beware that if you use the git version, it may break, it may not even compile, etc.
We do not recommend you use this, but if you want the latest changes or there is no suitable [crates.io](https://crates.io) version, this would work.

### Usage
#### Broker API

Create a client:
```rust,no_run
# use alpaca_rs::prelude::*;

let auth: BrokerAuth = BrokerAuth { key: std::env::var("ALPACA_BROKER_KEY").unwrap().into() };
let client = BrokerClient::new_sandbox(auth);
```

Make an account:
```rust,no_run
use alpaca_rs::prelude::*;

fn get_contact() -> Contact { todo!() }
fn get_identity() -> Identity { todo!() }

async fn run() -> Result<(), AlpacaError> {
    let client = BrokerClient::new_sandbox(BrokerAuth { key: std::env::var("ALPACA_BROKER_KEY").unwrap().into() });
    let account = client.create_account(get_contact(), get_identity()).execute().await?;
    println!("Created account: {account:#?}");
    Ok(())
}
```
