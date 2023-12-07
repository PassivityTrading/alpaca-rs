use super::*;

mod stock;

pub use stock::*;

pub trait MarketDataEndpoint: Endpoint {
    fn base_url(&self, client: &TradingClient) -> Url {
        client.market_data_base_url.clone()
    }
}
