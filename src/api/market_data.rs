use super::{*, trading::{TradingAuth, TraderMiddleware}};
use std::ops::RangeBounds;

#[cfg(feature = "market-data-live")]
pub mod live;
mod stock;

pub use stock::*;

// No API /version because its different on some endpoints
/// The live url for the Market Data API
const MARKET_PROD: &str = "https://data.alpaca.markets";
/// The sandbox url for the Market Data API
#[allow(dead_code)] // FIXME
const MARKET_SANDBOX: &str = "https://data.sandbox.alpaca.markets";

/// This client provides access to a "standalone" account on the Alpaca brokerage.
#[must_use = "A client does not do anything unless you execute endpoints with it yourself"]
pub struct MarketDataClient(HttpClient<super::trading::TraderMiddleware>);

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash, ClientEndpoint)]
#[endpoint(Get "/v2/clock" in MarketDataClient -> Clock)]
pub struct GetClock;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, ClientEndpoint)]
#[endpoint(Get(query) "/v2/calendar" in MarketDataClient -> Calendar)]
pub struct GetCalendar {
    pub start: Option<Date>,
    pub end: Option<Date>,
    pub date_type: DateType,
}

impl MarketDataClient {
    pub fn new(auth: TradingAuth, base_url: Url) -> Self {
Self(HttpClient::new_with(TraderMiddleware(auth)).with_base_url(base_url))
    }
    pub fn new_live(auth: TradingAuth) -> Self { Self::new(auth, MARKET_PROD.parse().unwrap()) }
    pub fn new_sandbox(auth: TradingAuth) -> Self { Self::new(auth, MARKET_SANDBOX.parse().unwrap()) }

    pub async fn execute<T: ClientEndpoint<Context = Self, Error = Error>>(
        &self,
        endpoint: T,
    ) -> Result<T::Output> {
        endpoint.run(self).await
    }

    /// Wait for the market to open.
    /// If the market is open, this will return immediately (excluding getting the clock data from
    /// Alpaca).
    pub async fn await_market_open(&self) -> Result<()> {
        trace!("Awaiting market opening.");
        let clock = self.get_clock().await?;
        if clock.is_open {
            trace!("Market is already open, not waiting.");
            return Ok(());
        }

        let wait = clock.next_open - clock.timestamp;
        trace!(
            "Waiting for market opening - {}h {}m left (until {})",
            wait.num_hours(),
            wait.num_minutes() - (wait.num_hours() * 60),
            clock.next_open.naive_utc()
        );
        async_std::task::sleep(wait.to_std().expect("duration to be non-negative")).await;

        Ok(())
    }

    pub async fn get_clock(&self) -> Result<Clock> {
        self.execute(GetClock).await
    }
    pub async fn get_calendar(
        &self,
        date: impl RangeBounds<Date>,
        date_type: DateType,
    ) -> Result<Calendar> {
        use std::collections::Bound;

        GetCalendar {
            start: if let Bound::Included(start) = date.start_bound() {
                Some(*start)
            } else {
                None
            },
            end: if let Bound::Included(end) = date.end_bound() {
                Some(*end)
            } else {
                None
            },
            date_type,
        }
        .run(self).await
    }
}

impl HttpClientContext for MarketDataClient {
    type Error = Error;

    fn new_request(&self, method: Method, url: &str) -> Request {
        self.0.new_request(method, url)
    }

    async fn run_request(&self, request: Request) -> Result<Response, Self::Error> {
        self.0.run_request(request).await
    }
}
