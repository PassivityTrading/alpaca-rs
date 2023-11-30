# alpaca-rs, an (unofficial) Rust SDK for the Alpaca API

## Features

- Ease of use - you just create a client execute requests
- Cross-platformness - can run on most platforms that Rust can run on - x86, aarch64, wasm (any platform that [`reqwest`](https://crates.io/crates/reqwest) can run on)
- 🔥BLAZINGLY 🚀 FAST🧨 - uses [`reqwest`](https://crates.io/crates/reqwest) to make requests, which is also very blazingly fast
- Interoperable - does not depend on any async runtime - do you want to use [`tokio`](https://crates.io/crates/tokio)? Go ahead. Prefer [`async-std`](https://crates.io/crates/async-std)? That's also an option. Literally anything that can poll a future will work.

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

Beware that if you use the git version, it may break, it may explode, etc. It is not recommended to use this, but if you want the latest changes or there is no suitable [crates.io](https://crates.io) version, this would be suitable.
