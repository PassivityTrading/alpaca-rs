//! An implementation of the Alpaca [Broker API](https://docs.alpaca.markets/docs/about-broker-api).
use super::*;

mod accounts;
mod funding;
mod trading;

pub use accounts::*;
pub use funding::*;
pub use trading::*;

/// The credentials used to authenticate with the Alpaca [Broker API](https://docs.alpaca.markets/docs/about-broker-api).
///
/// The key is a [`String`], that means you don't need to encode it as Base64 because this library does that automatically.
pub struct BrokerAuth {
    pub key: String,
}

struct BrokerMiddleware(BrokerAuth);

impl Service for BrokerMiddleware {
    type Error = Error;
    type Context = ();
}

impl Middleware for BrokerMiddleware {
    async fn call(&self, mut request: Request) -> Result<Response, Self::Error> {
        request.append_header(
            AUTHORIZATION,
            format!(
                "Basic {}",
                base64::engine::general_purpose::STANDARD.encode(&self.0.key)
            ),
        );

        Ok(NoMiddleware.call(request).await?)
    }
}

/// This structure provides access to the Alpaca [Broker API](https://docs.alpaca.markets/docs/about-broker-api).
///
/// For each available endpoint, there is a method on this struct, for example for
/// [`api::broker::GetAllAccounts`] there is [`BrokerClient::get_all_accounts`], etc.
///
/// If you want to execute endpoints directly, use the [`BrokerClient::execute`] method.
///
/// If you want to access a specific account's data or scope the requests to that specific account,
/// consider using the [`BrokerClient::account`] method, which returns an [`AccountView`].
#[must_use = "A client does not do anything unless you execute endpoints with it yourself"]
pub struct BrokerClient(HttpClient<BrokerMiddleware>);

/// The production/live url for the [Broker API](https://docs.alpaca.markets/docs/about-broker-api).
const BROKER_PROD: &str = "https://broker-api.alpaca.markets/v1";
/// The [sandbox](https://docs.alpaca.markets/docs/integration-setup-with-alpaca#sandbox) base url for the broker api.
const BROKER_SANDBOX: &str = "https://broker-api.sandbox.alpaca.markets/v1";

impl BrokerClient {
    /// Creates a new client configured with the live base url for the broker api.
    ///
    /// # See also
    /// [`BrokerClient::new`] for configuring the client to use your own base url
    /// [`BrokerClient::new_sandbox`] for creating a client configured to use the testing
    /// environment (the "sandbox")
    pub fn new_live(auth: BrokerAuth) -> Self {
        Self::new(auth, BROKER_PROD.parse().unwrap())
    }

    pub fn new_sandbox(auth: BrokerAuth) -> Self {
        Self::new(auth, BROKER_SANDBOX.parse().unwrap())
    }

    pub fn new(auth: BrokerAuth, base_url: Url) -> Self {
        Self(HttpClient::new_with(BrokerMiddleware(auth)).with_base_url(base_url))
    }

    pub async fn execute<T: ClientEndpoint<Context = Self, Error = Error>>(
        &self,
        endpoint: T,
    ) -> Result<T::Output> {
        endpoint.run(self).await
    }

    pub async fn account(&self, id: &str) -> AccountView {
        AccountView::new(
            id.to_owned(),
            BrokerMiddleware(BrokerAuth {
                key: self.0.get_middleware().0.key.to_owned(),
            }),
            self.0.base_url().cloned().unwrap(),
        )
    }
}

impl HttpClientContext for BrokerClient {
    type Error = Error;

    fn new_request(&self, method: Method, url: &str) -> Request {
        self.0.new_request(method, url)
    }

    async fn run_request(&self, request: Request) -> Result<Response, Self::Error> {
        self.0.run_request(request).await
    }
}
