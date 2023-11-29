use super::*;

/// Get account details.
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

/// Order amount (number of shares or dollar amount).
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

/// Create an order.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrder {
    /// The symbol/ticker of the stock being traded.
    pub symbol: String,
    /// Either the quantity or the dollar amount to trade.
    #[serde(flatten)]
    pub amount: OrderAmount,
    /// Buy or sell.
    pub side: OrderSide,
    /// Order type. Includes market, limit, stop, etc. orders.
    #[serde(flatten, rename = "type")]
    pub kind: OrderType,
    // TODO explain
    pub time_in_force: OrderTif,
    /// Specifies if the order is allowed to be processed in extended hours.
    pub extended_hours: bool,
    // TODO explain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    // TODO explain
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

/// Create an order on behalf of an account in the Broker API.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrderBroker {
    /// The symbol/ticker of the stock being traded.
    pub symbol: String,
    /// Either the quantity or the dollar amount to trade.
    #[serde(flatten)]
    pub amount: OrderAmount,
    /// Buy or sell.
    pub side: OrderSide,
    /// Order type. Includes market, limit, stop, etc. orders.
    #[serde(rename = "type", flatten)]
    pub kind: OrderType,
    // TODO(filter) explain
    pub time_in_force: OrderTif,
    /// Specifies if the order is allowed to be processed in extended hours.
    pub extended_hours: bool,
    // TODO explain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    // TODO explain
    pub order_class: OrderClass,
    /// The commission that the broker takes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub commission: Option<f64>,
    // TODO explain
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub commission_bps: Option<f64>,
    // TODO explain [no official explanation]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    // TODO explain [no official explanation]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    // TODO explain [no official explanation]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtag: Option<String>,
    // TODO explain [no official explanation]
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
