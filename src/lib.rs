#![doc = include_str!("../README.md")]
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

use std::{future::Future, ops::Deref};

use base64::Engine;
use model::Account;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Method, Url,
};
use serde::de::DeserializeOwned;

pub mod api;
pub mod model;

pub struct BrokerAuth {
    pub key: Vec<u8>,
}

#[must_use = "A client does not do anything unless you execute endpoints with it yourself"]
pub struct BrokerClient {
    pub reqwest: reqwest::Client,
    pub base_url: Url,
    auth: BrokerAuth,
}

/// The production/live url for the [Trading API](https://docs.alpaca.markets/docs/trading-api)
const TRADING_PROD: &str = "https://api.alpaca.markets";
/// see [Paper Trading](https://docs.alpaca.markets/docs/paper-trading)
const TRADING_PAPER: &str = "https://paper-api.alpaca.markets";
/// The production/live url for the [Broker API](https://docs.alpaca.markets/docs/about-broker-api)
const BROKER_PROD: &str = "https://broker-api.alpaca.markets/v1";
/// The [sandbox](https://docs.alpaca.markets/docs/integration-setup-with-alpaca#sandbox) base url for the broker api
const BROKER_SANDBOX: &str = "https://broker-api.sandbox.alpaca.markets/v1";

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

    fn br_auth(&self) -> String {
        format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(&self.auth.key)
        )
    }

    pub async fn execute<T: Endpoint + BrokerEndpoint>(&self, endpoint: T) -> Result<T::Result> {
        let request = endpoint
            .configure(self.reqwest.request(
                endpoint.method(),
                endpoint.base_url(self).join(endpoint.url()).unwrap(),
            ))
            .header(AUTHORIZATION, self.br_auth());

        T::deserialize(request.send().await?).await
    }

    pub async fn execute_trading<T: Endpoint + BrokerTradingEndpoint>(
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
                        .join(&endpoint.br_url(account_id))
                        .unwrap(),
                ),
            )
            .header(AUTHORIZATION, self.br_auth());
        T::deserialize(request.send().await?).await
    }

    pub async fn account(&self, id: &str) -> Result<AccountView<'_>> {
        Ok(AccountView {
            data: self.execute_trading(api::trading::GetAccount, id).await?,
            client: self,
        })
    }
}

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
            .execute_trading(api::trading::GetAccount, &self.data.id)
            .await?;

        Ok(())
    }

    pub async fn execute<T: Endpoint + BrokerTradingEndpoint>(
        &self,
        endpoint: T,
    ) -> Result<T::Result> {
        self.client.execute_trading(endpoint, &self.data.id).await
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

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
    type Result;

    fn method(&self) -> Method;
    fn url(&self) -> &'static str;
    fn configure(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder;
    #[doc(hidden)]
    fn deserialize(
        response: reqwest::Response,
    ) -> impl Future<Output = Result<Self::Result>> + 'static;
}

pub trait BrokerEndpoint: Endpoint {
    fn base_url(&self, client: &BrokerClient) -> Url {
        client.base_url.clone()
    }
}

pub struct TradingAuth {
    pub key: String,
    pub secret: String,
}

#[must_use = "A client does not do anything unless you execute endpoints with it yourself"]
pub struct TradingClient {
    pub reqwest: reqwest::Client,
    pub base_url: Url,
    auth: TradingAuth,
}

impl TradingClient {
    pub fn new_live(auth: TradingAuth) -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            base_url: TRADING_PROD.parse().unwrap(),
            auth,
        }
    }

    pub fn new_paper(auth: TradingAuth) -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            base_url: TRADING_PAPER.parse().unwrap(),
            auth,
        }
    }

    pub fn new(auth: TradingAuth, base_url: Url) -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            base_url,
            auth,
        }
    }

    pub fn with_reqwest(self, reqwest: reqwest::Client) -> Self {
        Self { reqwest, ..self }
    }

    fn auth_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let _ = headers.insert("APCA-API-KEY-ID", self.auth.key.parse().unwrap());
        let _ = headers.insert("APCA-API-SECRET-KEY", self.auth.secret.parse().unwrap());
        headers
    }

    pub async fn execute<T: TradingEndpoint>(&self, endpoint: T) -> Result<T::Result> {
        let request = endpoint
            .configure(self.reqwest.request(
                endpoint.method(),
                endpoint.base_url(self).join(endpoint.url()).unwrap(),
            ))
            .headers(self.auth_headers());

        T::deserialize(request.send().await?).await
    }
}

pub trait TradingEndpoint: Endpoint {
    fn base_url(&self, client: &TradingClient) -> Url {
        client.base_url.clone()
    }
}

pub trait BrokerTradingEndpoint: Endpoint + BrokerEndpoint {
    fn br_url(&self, _account_id: &str) -> String {
        self.url().to_owned()
    }
}

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

                /// Builds the endpoint data. If you use this, you would have to manually pass this
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
    (@mode_marker $name:ident account) => { with_builder!(@mode_marker $name broker); };
    (@mode_marker $name:ident broker) => { impl BrokerEndpoint for $name {} };
    (@mode_marker $name:ident trading) => { impl TradingEndpoint for $name {} };
    (@mode) => { with_builder!(@mode trading) };
    (@mode trading) => { TradingClient };
    (@mode broker) => { BrokerClient };
    (@mode account) => { AccountView<'a> };
    (@docmunch |$field:ident| #[doc = $($doc:tt)*] $(#$tail:tt)*) => { $($doc)* };
    (@docmunch |$field:ident| $(#$tail:tt)*) => (
        concat!("If you see this, then the ", stringify!($field), " has no documentation. Please report this, as it is either a bug in the internal `with_builder!` macro, or a bug in the endpoint defintion, or the documentation people are very lazy. Either way, please file an issue for this in the [GitHub repository](<https://github.com/PassivityTrading/alpaca-rs/issues/new>).")
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! endpoint {
    ($(#[$meta:meta])* $vis:vis struct $name:ident $({ $($fields:tt)* })?; $($imp:tt)*) => {
        #[derive(serde::Serialize, serde::Deserialize)]
        $(#[$meta])*
        $vis struct $name {$($fields)*}
        $crate::endpoint!($($imp)*);
    };
    // GET "/account" = GetAccount => Account
    ($(impl $method:ident $url:literal = $name:ident => $result:ty$({ $configure:expr })?);*$(;)?) => {
        $(
        impl Endpoint for $name {
            type Result = $result;

            fn url(&self) -> &'static str {
                $url
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
        })*
    };
    (@configure $this:ident $why:ident) => ($why);
    (@configure $this:ident $why:ident $configure:expr) => {{
        fn force_specific<T>(this: &T, req: reqwest::RequestBuilder, lam: impl FnOnce(&T, reqwest::RequestBuilder) -> reqwest::RequestBuilder) -> reqwest::RequestBuilder { lam(this, req) }
        (force_specific::<Self>($this, $why, $configure)) }};
}
