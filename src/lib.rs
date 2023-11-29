use std::{future::Future, ops::Deref, pin::Pin};

use base64::Engine;
use model::Account;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Method, Url,
};
use serde::de::DeserializeOwned;

pub mod model;

pub struct BrokerAuth {
    pub key: Vec<u8>,
}

pub struct BrokerClient {
    pub reqwest: reqwest::Client,
    pub base_url: Url,
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

    pub fn with_base_url(self, base_url: Url) -> Self {
        Self { base_url, ..self }
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
            data: self.execute_trading(model::trading::GetAccount, id).await?,
            client: self,
        })
    }
}

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
            .execute_trading(model::trading::GetAccount, &self.data.id)
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
    ) -> Pin<Box<dyn Future<Output = Result<Self::Result>> + 'static>>;
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

pub struct TradingClient {
    pub reqwest: reqwest::Client,
    pub base_url: Url,
    auth: TradingAuth,
}

impl TradingClient {
    pub fn new(auth: TradingAuth) -> Self {
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

fn json_self<T: DeserializeOwned>(
    response: reqwest::Response,
) -> Pin<Box<dyn Future<Output = Result<T>> + 'static>> {
    Box::pin(async move { Ok(response.error_for_status()?.json::<T>().await?) })
}
