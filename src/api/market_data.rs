use super::*;

mod stock;
#[cfg(feature = "market-data-live")]
pub mod live;

pub use stock::*;

pub trait MarketDataEndpoint: Endpoint {
    fn base_url(&self, client: &TradingClient) -> Url {
        client.market_data_base_url.clone()
    }
}
