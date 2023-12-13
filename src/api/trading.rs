use super::*;

mod assets;
mod orders;
mod positions;

pub use assets::*;
pub use orders::*;
pub use positions::*;

/// The production/live url for the [Trader API](https://docs.alpaca.markets/docs/trading-api).
const TRADING_PROD: &str = "https://api.alpaca.markets";
/// The [Paper Trading](https://docs.alpaca.markets/docs/paper-trading) url for the Trader API.
const TRADING_PAPER: &str = "https://paper-api.alpaca.markets";

/// The credentials for authorizing on the Trader API.
pub struct TradingAuth {
    pub key: String,
    pub secret: String,
}

pub(crate) struct TraderMiddleware(pub(crate) TradingAuth);

impl Service for TraderMiddleware {
    type Error = Error;
    type Context = ();
}

impl Middleware for TraderMiddleware {
    async fn call(&self, mut request: Request) -> Result<Response, <Self as Service>::Error> {
        request.append_header("APCA-API-KEY-ID", &self.0.key);
        request.append_header("APCA-API-SECRET-KEY", &self.0.secret);

        trace!("{request:?}");

        Ok(match NoMiddleware.call(request).await? {
            res if res.status().is_success() => res,
            mut other => return Err(http_types::Error::from_str(other.status(), format!("status was not successful: {other:?}, {}", other.body_string().await?)).into())
        })
    }
}

/// This client provides access to a "standalone" account on the Alpaca brokerage.
#[must_use = "A client does not do anything unless you execute endpoints with it yourself"]
pub struct TradingClient(HttpClient<TraderMiddleware>);

impl TradingClient {
    pub fn new_live(auth: TradingAuth) -> Self {
        Self::new(auth, TRADING_PROD.parse().unwrap())
    }

    pub fn new_paper(auth: TradingAuth) -> Self {
        Self::new(auth, TRADING_PAPER.parse().unwrap())
    }

    pub fn new(auth: TradingAuth, base_url: Url) -> Self {
        Self(HttpClient::new_with(TraderMiddleware(auth)).with_base_url(base_url))
    }
    
    /// Gets the account data for this trading account.
    pub async fn get_account(&self) -> Result<Account> {
        self.execute(GetAccount).await
    }

    pub async fn get_clock(&self) -> Result<Clock> {
        self.execute(GetClock).await
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

    pub async fn execute<T: ClientEndpoint<Context = Self, Error = Error>>(
        &self,
        endpoint: T,
    ) -> Result<T::Output> {
        endpoint.run(self).await
    }
}

impl HttpClientContext for TradingClient {
    type Error = Error;

    fn new_request(&self, method: Method, url: &str) -> Request {
        self.0.new_request(method, url)
    }

    async fn run_request(&self, request: Request) -> Result<Response, Self::Error> {
        self.0.run_request(request).await
    }
}

/// Get account details.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash, ClientEndpoint)]
#[endpoint(Get "/v2/account" in TradingClient -> Account)]
pub struct GetAccount;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash, ClientEndpoint)]
#[endpoint(Get "/v2/clock" in TradingClient -> Clock)]
pub struct GetClock;
