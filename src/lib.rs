use std::{future::Future, pin::Pin};

use reqwest::Url;

pub mod model;

pub struct BrokerClient {
    pub reqwest: reqwest::Client,
    pub trading_base_url: Url,
    pub broker_base_url: Url,
}

const TRADING_PROD: &str = "https://api.alpaca.markets";
const TRADING_PAPER: &str = "https://paper-api.alpaca.markets";
const BROKER_PROD: &str = "https://broker-api.alpaca.markets/v1";
const BROKER_SANDBOX: &str = "https://broker-api.sandbox.alpaca.markets/v1";

impl BrokerClient {
    pub fn new_prod() -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            trading_base_url: Url::parse(TRADING_PROD).unwrap(),
            broker_base_url: Url::parse(BROKER_PROD).unwrap(),
        }
    }

    pub fn new_sandbox() -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            trading_base_url: Url::parse(TRADING_PAPER).unwrap(),
            broker_base_url: Url::parse(BROKER_SANDBOX).unwrap(),
        }
    }

    pub async fn execute<T: Endpoint + BrokerEndpoint>(&self, endpoint: T) -> Result<T::Result> {
        let request = endpoint.configure(self.reqwest.request(
            endpoint.method(),
            endpoint.base_url(self).join(endpoint.url()).unwrap(),
        ));

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

    fn method(&self) -> reqwest::Method;
    fn url(&self) -> &'static str;
    fn configure(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder;
    #[doc(hidden)]
    fn deserialize(
        response: reqwest::Response,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Result>> + 'static + Send + Sync>>;
}

pub trait BrokerEndpoint: Endpoint {
    fn base_url(&self, client: &BrokerClient) -> Url {
        client.broker_base_url.clone()
    }
}



