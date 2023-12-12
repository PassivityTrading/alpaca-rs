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

        Ok(NoMiddleware.call(request).await?)
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
#[endpoint(Get "/account" in TradingClient -> Account)]
pub struct GetAccount;
