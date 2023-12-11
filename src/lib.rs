#![doc = include_str!("../README.md")]
#![doc(html_favicon_url = "https://files.alpaca.markets/webassets/favicon.ico")]
// perf
#![feature(return_position_impl_trait_in_trait)]
#![deny(clippy::all)]
#![allow(
    // don't care
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    // required by alpaca api
    clippy::struct_excessive_bools,
    // allows for better dx
    clippy::wildcard_imports
)]
#![forbid(clippy::missing_safety_doc)]
// #![deny(missing_docs)]

use api::market_data::MarketDataEndpoint;
use std::{borrow::Cow, future::Future, ops::Deref};
use tracing::log::*;

use base64::Engine;
use chrono::{NaiveDate, Utc};
use model::Account;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Method, Url,
};
use serde::de::DeserializeOwned;

type Date = NaiveDate;
type DateTime<Tz = Utc> = chrono::DateTime<Tz>;
pub use chrono;

pub mod api;
pub mod model;
pub mod pagination;

/// The credentials used to authenticate with the Alpaca [Broker API](https://docs.alpaca.markets/docs/about-broker-api).
///
/// The key is a [`String`], that means you don't need to encode it as Base64 because this library does that automatically.
pub struct BrokerAuth {
    pub key: String,
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
pub struct BrokerClient {
    pub reqwest: reqwest::Client,
    pub base_url: Url,
    auth: BrokerAuth,
}

/// The production/live url for the [Trader API](https://docs.alpaca.markets/docs/trading-api).
const TRADING_PROD: &str = "https://api.alpaca.markets";
/// The [Paper Trading](https://docs.alpaca.markets/docs/paper-trading) url for the Trader API.
const TRADING_PAPER: &str = "https://paper-api.alpaca.markets";
/// The production/live url for the [Broker API](https://docs.alpaca.markets/docs/about-broker-api).
const BROKER_PROD: &str = "https://broker-api.alpaca.markets/v1";
/// The [sandbox](https://docs.alpaca.markets/docs/integration-setup-with-alpaca#sandbox) base url for the broker api.
const BROKER_SANDBOX: &str = "https://broker-api.sandbox.alpaca.markets/v1";
// No API /version because its different on some endpoints
/// The live url for the Market Data API
const MARKET_PROD: &str = "https://data.alpaca.markets";
/// The sandbox url for the Market Data API
#[allow(dead_code)] // FIXME
const MARKET_SANDBOX: &str = "https://data.sandbox.alpaca.markets";

impl BrokerClient {
    /// Creates a new client configured with the live base url for the broker api.
    ///
    /// # See also
    /// [`BrokerClient::new`] for configuring the client to use your own base url
    /// [`BrokerClient::new_sandbox`] for creating a client configured to use the testing
    /// environment (the "sandbox")
    pub fn new_live(auth: BrokerAuth) -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            base_url: Url::parse(BROKER_PROD).unwrap(),
            auth,
        }
    }

    pub fn new_sandbox(auth: BrokerAuth) -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            base_url: Url::parse(BROKER_SANDBOX).unwrap(),
            auth,
        }
    }

    pub fn with_reqwest(self, reqwest: reqwest::Client) -> Self {
        Self { reqwest, ..self }
    }

    pub fn new(auth: BrokerAuth, base_url: Url) -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            base_url,
            auth,
        }
    }

    fn authorization_header(&self) -> String {
        format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(&self.auth.key)
        )
    }

    pub async fn execute<T: Endpoint + BrokerEndpoint>(&self, endpoint: T) -> Result<T::Result> {
        let request = endpoint
            .configure(self.reqwest.request(
                endpoint.method(),
                endpoint.base_url(self).join(&endpoint.url()).unwrap(),
            ))
            .header(AUTHORIZATION, self.authorization_header());

        T::deserialize(request.send().await?).await
    }

    pub async fn execute_with_account<T: Endpoint + AccountEndpoint>(
        &self,
        endpoint: T,
        account_id: &str,
    ) -> Result<T::Result> {
        let request = endpoint
            .configure(
                self.reqwest.request(
                    endpoint.method(),
                    endpoint
                        .base_url(self)
                        .join(&endpoint.broker_url(account_id))
                        .unwrap(),
                ),
            )
            .header(AUTHORIZATION, self.authorization_header());
        T::deserialize(request.send().await?).await
    }

    pub async fn account(&self, id: &str) -> Result<AccountView<'_>> {
        Ok(AccountView {
            data: self
                .execute_with_account(api::trading::GetAccount, id)
                .await?,
            client: self,
        })
    }
}

/// An account view is like a [`BrokerClient`], but scoped to a single account.
///
/// There is a caveat to using this - when created, it fetches the account data and stores it. You
/// might not want this behavior, for example if you are doing one operation or don't need the
/// account data at all. In that case, use the [`BrokerClient::execute_account`] method instead,
/// providing it the endpoint's data and an account ID to act on.
#[must_use = "An account view does not do anything unless you execute endpoints with it yourself"]
pub struct AccountView<'a> {
    pub data: Account,
    client: &'a BrokerClient,
}

impl<'a> Deref for AccountView<'a> {
    type Target = Account;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a> AccountView<'a> {
    pub async fn refetch(&mut self) -> Result<()> {
        self.data = self
            .client
            .execute_with_account(api::trading::GetAccount, &self.data.id)
            .await?;

        Ok(())
    }

    pub async fn execute<T: Endpoint + AccountEndpoint>(&self, endpoint: T) -> Result<T::Result> {
        self.client
            .execute_with_account(endpoint, &self.data.id)
            .await
    }
}

/// An Alpaca [`Result`](core::result::Result).
/// This is just an alias to `Result<T, Error>`.
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// An HTTP error (a "bad request" status code or a network error),
/// or a serialization error (which means the response from the API was not valid JSON).
#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(alpaca::http_error))]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    #[diagnostic(code(alpaca::json_error))]
    Json(#[from] serde_json::Error),
}

/// An Alpaca endpoint. Has methods to configure a request and deserialize a response.
pub trait Endpoint {
    /// The output type of this endpoint.
    /// Usually this is the object that an endpoint creates or modifies.
    type Result;

    #[doc(hidden)]
    fn method(&self) -> Method;
    #[doc(hidden)]
    fn url(&self) -> Cow<'static, str>;
    #[doc(hidden)]
    fn configure(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder;
    #[doc(hidden)]
    fn deserialize(
        response: reqwest::Response,
    ) -> impl Future<Output = Result<Self::Result>> + 'static;
}

#[doc(hidden)]
pub trait BrokerEndpoint {
    fn base_url(&self, client: &BrokerClient) -> Url {
        client.base_url.clone()
    }
}

/// The credentials for authorizing on the Trader API.
pub struct TradingAuth {
    pub key: String,
    pub secret: String,
}

// does not implment Debug to not leak creds
/// This client provides access to a "standalone" account on the Alpaca brokerage.
#[must_use = "A client does not do anything unless you execute endpoints with it yourself"]
pub struct TradingClient {
    pub reqwest: reqwest::Client,
    pub base_url: Url,
    pub market_data_base_url: Url,
    // private for disallowing unpredictable modification and generally credential leaks
    auth: TradingAuth,
}

impl TradingClient {
    pub fn new_live(auth: TradingAuth) -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            base_url: TRADING_PROD.parse().unwrap(),
            market_data_base_url: MARKET_PROD.parse().unwrap(),
            auth,
        }
    }

    pub fn new_paper(auth: TradingAuth) -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            base_url: TRADING_PAPER.parse().unwrap(),
            // FIXME for some reason sandbox just refuses to work?
            market_data_base_url: MARKET_PROD.parse().unwrap(),
            auth,
        }
    }

    pub fn new(auth: TradingAuth, base_url: Url) -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            base_url,
            market_data_base_url: MARKET_PROD.parse().unwrap(),
            auth,
        }
    }

    pub fn new_full(auth: TradingAuth, base_url: Url, market_data_base_url: Url) -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            base_url,
            market_data_base_url,
            auth,
        }
    }

    pub fn with_reqwest(self, reqwest: reqwest::Client) -> Self {
        Self { reqwest, ..self }
    }

    #[cfg(feature = "tokio")]
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
        tokio::time::sleep(wait.to_std().expect("duration to be non-negative")).await;

        Ok(())
    }

    fn auth_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let _ = headers.insert("APCA-API-KEY-ID", self.auth.key.parse().unwrap());
        let _ = headers.insert("APCA-API-SECRET-KEY", self.auth.secret.parse().unwrap());
        headers
    }

    pub async fn execute<T: Endpoint + TradingEndpoint + serde::ser::Serialize + std::fmt::Debug>(&self, endpoint: T) -> Result<T::Result> {
        trace!("[trading] running {endpoint:?} = {}", serde_json::to_string(&endpoint)?);
        let request = endpoint
            .configure(self.reqwest.request(
                endpoint.method(),
                endpoint.base_url(self).join(&endpoint.url()).unwrap(),
            ))
            .headers(self.auth_headers());

        T::deserialize(request.send().await?).await
    }

    pub async fn execute_market<T: Endpoint + MarketDataEndpoint + std::fmt::Debug>(
        &self,
        endpoint: T,
    ) -> Result<T::Result> {
        trace!("[market_data] running {endpoint:?}");
        let request = endpoint.configure(
            self.reqwest
                .request(
                    endpoint.method(),
                    endpoint.base_url(self).join(&endpoint.url()).unwrap(),
                )
                .headers(self.auth_headers()),
        );

        T::deserialize(request.send().await?).await
    }
}

#[doc(hidden)]
pub trait TradingEndpoint {
    fn base_url(&self, client: &TradingClient) -> Url {
        client.base_url.clone()
    }
}

#[doc(hidden)]
pub trait AccountEndpoint: Endpoint + BrokerEndpoint {
    fn broker_url(&self, _account_id: &str) -> String {
        self.url().into_owned()
    }
}

/// A convenience function for making sure the response is not an error
/// and deserializing it as JSON into `T`.
async fn json_self<T: DeserializeOwned>(response: reqwest::Response) -> Result<T> {
    Ok(response.error_for_status()?.json::<T>().await?)
}

#[doc(hidden)]
#[macro_export]
/// Internal macro used for making endpoints with builders.
macro_rules! with_builder {
    ($(|$mode:ident|)? $(#[$meta:meta])*$vis:vis struct $name:ident { $($(#$fm:tt)* $fv:vis $field:ident: $fty:ty),*$(,)? }) => {
        paste::paste! {
            $(#[$meta])*$vis struct $name { $($(#$fm)* $fv $field: $fty,)* }

            #[doc = "Builder for the [`"[<$name>]"`] endpoint."]
            #[must_use = "A builder does not do anything unless you use `.execute()` it"]
            $vis struct [<$name Builder>]<'a>(&'a with_builder!(@mode $($mode)?), $name);
            impl [<$name Builder>]<'_> {
                /// Executes this request.
                pub async fn execute(self) -> Result<<$name as Endpoint>::Result> {
                    self.0.execute(self.1).await
                }

                /// Builds the endpoint data. If you use this, you would have to manually give this
                /// built endpoint to the client.
                ///
                /// You most likely don't need this, and instead need [`Self::execute`].
                #[must_use]
                pub fn build(self) -> $name {
                    self.1
                }

                $(
                #[doc = with_builder!(@docmunch |$field| $(#$fm)*)]
                pub fn $field(mut self, __real: impl Into<$fty>) -> Self {
                    self.1.$field = Into::<$fty>::into(__real);
                    self
                })*
            }

            with_builder!(@mode_marker $name $($mode)?);
        }
    };
    (@mode_marker $name:ident) => {};
    (@mode_marker $name:ident market_data) => {
        impl $crate::api::market_data::MarketDataEndpoint for $name {}
        impl TradingEndpoint for $name {}
    };
    (@mode_marker $name:ident account) => { with_builder!(@mode_marker $name broker); };
    (@mode_marker $name:ident broker) => { impl BrokerEndpoint for $name {} };
    (@mode_marker $name:ident trading) => { impl TradingEndpoint for $name {} };
    (@mode) => { with_builder!(@mode trading) };
    (@mode market_data) => { TradingClient };
    (@mode trading) => { TradingClient };
    (@mode broker) => { BrokerClient };
    (@mode account) => { AccountView<'a> };
    (@docmunch |$field:ident| #[doc = $($doc:tt)*] $(#$tail:tt)*) => { $($doc)* };
    (@docmunch |$field:ident| $(#$tail:tt)*) => (
        ""
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! endpoint {
    ($(impl $method:ident $url:tt = $name:ident $(=> $result:ty)?$({ $configure:expr })?$(| $id:ident $(($ur:expr))?)*);*$(;)?) => {
        $(
            impl Endpoint for $name {
                type Result = endpoint!(@result $($result)?);

                fn url(&self) -> std::borrow::Cow<'static, str> {
                    endpoint!(@url |self| $url)
                }

                fn method(&self) -> reqwest::Method {
                    reqwest::Method::$method
                }

                fn configure(&self, __req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
                    endpoint!(@configure self __req $($configure)?)
                }

                fn deserialize(response: reqwest::Response) -> impl Future<Output = Result<Self::Result>> + 'static {
                    $crate::json_self(response)
                }
            }
            $(endpoint!(@impl_thing $name $id $($ur)?);)*
        )*
    };
    (@impl_thing $name:ident broker) => { impl BrokerEndpoint for $name {} };
    (@impl_thing $name:ident trading) => { impl TradingEndpoint for $name {} };
    (@impl_thing $name:ident market_data) => { impl $crate::api::market_data::MarketDataEndpoint for $name {} };
    (@impl_thing $name:ident account $($br_url:expr)?) => {
        impl AccountEndpoint for $name {
            fn broker_url(&self, account_id: &str) -> String {
                fn force_specific<T>(this: &T, account_id: &str, lam: impl FnOnce(&T, &str) -> String) -> String { lam(this, account_id) }
                force_specific(self, account_id, endpoint!(@br_url $($br_url)?))
            }
        }
    };
    (@result $result:ty) => ($result);
    (@result) => (());
    (@br_url) => (|this, account_id| format!("/accounts/{account_id}{}", &this.url()));
    (@br_url $br_url:expr) => ($br_url);
    (@url |$this:ident| $url:literal) => (std::borrow::Cow::Borrowed($url));
    (@url |$this:ident| ($url:expr)) => ({fn force_specific<T>(this: &T, lam: impl FnOnce(&T) -> String) -> String { lam(this) }
        std::borrow::Cow::Owned(force_specific($this, $url))});
    (@configure $this:ident $why:ident) => ($why);
    (@configure $this:ident $why:ident $configure:expr) => {{
        fn force_specific<T>(this: &T, req: reqwest::RequestBuilder, lam: impl FnOnce(&T, reqwest::RequestBuilder) -> reqwest::RequestBuilder) -> reqwest::RequestBuilder { lam(this, req) }
        (force_specific::<Self>($this, $why, $configure)) }};
}

/// `use alpaca_rs::prelude::*;` to import the most commonly used types and clients.
pub mod prelude {
    pub use crate::model::*;
    pub use crate::{BrokerAuth, BrokerClient, Error as AlpacaError, TradingAuth, TradingClient};
}

/// A trait for identifying a single object in the API.
/// Required for pagination, as it needs the last item's ID.
pub trait Identifiable {
    fn id(&self) -> String;
    fn next_page_token(&self) -> Option<String> {
        Some(self.id())
    }
}
