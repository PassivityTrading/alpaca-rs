use std::{future::Future, pin::Pin};

use base64::Engine;
use reqwest::{header::AUTHORIZATION, Method, Url};
use serde::de::DeserializeOwned;

pub mod model;

pub struct BrokerAuth {
    pub key: String,
}

pub struct BrokerClient {
    pub reqwest: reqwest::Client,
    pub broker_base_url: Url,
    auth: BrokerAuth,
}

const TRADING_PROD: &str = "https://api.alpaca.markets";
const TRADING_PAPER: &str = "https://paper-api.alpaca.markets";
const BROKER_PROD: &str = "https://broker-api.alpaca.markets/v1";
const BROKER_SANDBOX: &str = "https://broker-api.sandbox.alpaca.markets/v1";

impl BrokerClient {
    pub fn new_prod(auth: BrokerAuth) -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            broker_base_url: Url::parse(BROKER_PROD).unwrap(),
            auth,
        }
    }

    pub fn new_sandbox(auth: BrokerAuth) -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            broker_base_url: Url::parse(BROKER_SANDBOX).unwrap(),
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
    ) -> Pin<Box<dyn Future<Output = Result<Self::Result>> + 'static>>;
}

pub trait BrokerEndpoint: Endpoint {
    fn base_url(&self, client: &BrokerClient) -> Url {
        client.broker_base_url.clone()
    }
}

#[doc(hidden)]
pub trait Urls {
    fn broker_base_url(&self) -> Url;
    fn trading_base_url(&self) -> Url;
}

impl Urls for BrokerClient {
    fn broker_base_url(&self) -> Url {
        self.broker_base_url.clone()
    }
    fn trading_base_url(&self) -> Url {
        self.trading_base_url.clone()
    }
}

pub trait TradingEndpoint: Endpoint {
    fn base_url(&self, client: &impl Urls) -> Url {
        client.trading_base_url()
    }
}

pub trait BrokerTradingEndpoint: Endpoint + BrokerEndpoint {
    fn br_url(&self, _account_id: &str) -> String {
        self.url().to_owned()
    }
}

fn json_self<T: DeserializeOwned>(
    response: reqwest::Response,
) -> Pin<Box<dyn Future<Output = Result<T>> + 'static>> {
    Box::pin(async move { Ok(response.error_for_status()?.json::<T>().await?) })
}
