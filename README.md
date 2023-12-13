# alpaca-rs, an unofficial Rust SDK for the Alpaca API (WIP)

## Features

- Easy to use - Minimal setup and no boilerplate.
- Fast - Our own HTTP client, [Acril](https://github.com/PassivityTrading/acril), gives us better performance with less code.
- Cross-platform - can run on most platforms that Rust can run on, including WebAssembly.
- Interoperable - does not depend on any async runtime, so you can use whatever executor you want, like [Tokio](https://tokio.rs), [`actix-rt`](https://crates.io/crates/actix-rt), or [`async-std`'s own executor](https://docs.rs/async-std/latest/async_std/task/fn.spawn.html).
- Supports **all** Alpaca APIs - Broker, Trading and Market Data APIs.

## Guide

### Installing

#### Crates.io (not released yet)

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
use alpaca_rs::prelude::*;

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
