use super::*;

pub struct GetAccount;

impl Endpoint for GetAccount {
    type Result = Account;

    fn url(&self) -> &'static str {
        "/account"
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn configure(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request
    }
    fn deserialize(
        response: reqwest::Response,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Result>> + 'static>> {
        json_self(response)
    }
}

impl TradingEndpoint for GetAccount {}
impl BrokerEndpoint for GetAccount {}
impl BrokerTradingEndpoint for GetAccount {
    fn br_url(&self, account_id: &str) -> String {
        format!("accounts/{account_id}")
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum OrderAmount {
    #[serde(rename = "qty")]
    /// Number of shares.
    Quantity(#[serde_as(as = "DisplayFromStr")] f64),
    #[serde(rename = "notional")]
    /// Notional amount is the amount of stock in the currency of the account.
    Notional(#[serde_as(as = "DisplayFromStr")] f64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrder {
    pub symbol: String,
    #[serde(flatten)]
    pub amount: OrderAmount,
    pub side: OrderSide,
    #[serde(rename = "type")]
    #[serde(flatten)]
    pub kind: OrderType,
    pub time_in_force: OrderTif,
    pub extended_hours: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    pub order_class: OrderClass,
}

impl Endpoint for CreateOrder {
    type Result = Order;
    fn url(&self) -> &'static str {
        "/orders"
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn configure(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request.json(self)
    }
    fn deserialize(
        response: reqwest::Response,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Result>> + 'static>> {
        json_self(response)
    }
}

impl TradingEndpoint for CreateOrder {}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrderBroker {
    pub symbol: String,
    #[serde(flatten)]
    pub amount: OrderAmount,
    pub side: OrderSide,
    #[serde(rename = "type")]
    #[serde(flatten)]
    pub kind: OrderType,
    pub time_in_force: OrderTif,
    pub extended_hours: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    pub order_class: OrderClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub commission: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub commission_bps: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_fee_bps: Option<String>,
}

impl Endpoint for CreateOrderBroker {
    type Result = Order;

    fn url(&self) -> &'static str {
        "/orders"
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn configure(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request.json(self)
    }
    fn deserialize(
        response: reqwest::Response,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Result>> + 'static>> {
        json_self(response)
    }
}

impl BrokerEndpoint for CreateOrderBroker {}

impl BrokerTradingEndpoint for CreateOrderBroker {
    fn br_url(&self, account_id: &str) -> String {
        format!("accounts/{account_id}/orders")
    }
}
