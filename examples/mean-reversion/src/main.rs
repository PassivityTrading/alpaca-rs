use actix::prelude::*;
use alpaca_rs::{Result, TradingAuth, TradingClient};
use tracing::*;
use tracing_subscriber::EnvFilter;

struct Service {
    alpaca: TradingClient,
    wait_for_open: bool,
}

impl Service {
    async fn run(self) -> Result<()> {
        if self.wait_for_open {
            info!("Waiting for market opening...");
            self.alpaca.await_market_open().await?;
            info!("-- Market open!");
        } else {
            warn!("Not waiting for market open. Trades may not pass and/or profit.");
        }

        Ok(())
    }
}

#[actix_rt::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting mean-reversion algorithm");

    Service {
        alpaca: TradingClient::new_paper(TradingAuth {
            key: std::env::var("APCA_API_KEY").unwrap(),
            secret: std::env::var("APCA_SECRET_KEY").unwrap(),
        }),
        wait_for_open: !std::env::var("APCA_WAIT_OPEN").is_ok_and(|x| x == "0"),
    }
    .run()
    .await?;

    System::current().stop();

    Ok(())
}
