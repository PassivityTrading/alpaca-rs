use super::{
    trading::{TraderMiddleware, TradingAuth},
    *,
};
use std::ops::RangeBounds;

#[cfg(feature = "market-data-live")]
pub mod live;
mod stock;

pub use stock::*;

// No API /version because its different on some endpoints
/// The live url for the Market Data API
const MARKET_PROD: &str = "https://data.alpaca.markets/v2";
/// The sandbox url for the Market Data API
#[allow(dead_code)] // FIXME does not work currently
const MARKET_SANDBOX: &str = "https://data.sandbox.alpaca.markets/v2";

/// This client provides access to a "standalone" account on the Alpaca brokerage.
#[must_use = "A client does not do anything unless you execute endpoints with it yourself"]
pub struct MarketDataClient(HttpClient<super::trading::TraderMiddleware>);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, ClientEndpoint)]
#[endpoint(Get(query) "/calendar" in MarketDataClient -> Calendar)]
pub struct GetCalendar {
    pub start: Option<Date>,
    pub end: Option<Date>,
    pub date_type: DateType,
}

impl MarketDataClient {
    pub fn new(auth: TradingAuth, base_url: Url) -> Self {
        Self(HttpClient::new_with(TraderMiddleware(auth)).with_base_url(base_url))
    }
    pub fn new_live(auth: TradingAuth) -> Self {
        Self::new(auth, MARKET_PROD.parse().unwrap())
    }
    pub fn new_sandbox(auth: TradingAuth) -> Self {
        Self::new(auth, MARKET_SANDBOX.parse().unwrap())
    }

    pub async fn execute<T: ClientEndpoint<Context = Self, Error = Error>>(
        &self,
        endpoint: T,
    ) -> Result<T::Output> {
        endpoint.run(self).await
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
        .run(self)
        .await
    }
}

impl HttpClientContext for MarketDataClient {
    type Error = Error;

    fn new_request(&self, method: Method, url: &str) -> Request {
        // HACK for leading slashes in endpoint urls, the url parser does not like that when
        // joining so it just yeets out the api version from the base url (i.e.
        // api.alpaca.markets/v2 with the url /orders becomes api.alpaca.markets/orders).
        // this behavior is not very sensical but in order to allow stylish urls we can just slice
        // off the first char (i.e. the leading slash), and if others want to specify another api
        // version they could just have two (i.e. "//v2/orders").
        self.0.new_request(method, &url[1..])
    }

    async fn run_request(&self, request: Request) -> Result<Response, Self::Error> {
        self.0.run_request(request).await
    }
}
