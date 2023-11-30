use super::*;

/// Get account details.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GetAccount;

impl TradingEndpoint for GetAccount {}
impl BrokerEndpoint for GetAccount {}

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

with_builder! { |trading|
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
        pub order_class: OrderClass
    }
}

with_builder! { |account|
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
        pub swap_fee_bps: Option<String>
    }
}

endpoint! {
    impl GET "/account" = GetAccount => Account;
    impl POST "/orders" = CreateOrder => Order;
    impl POST "/orders" = CreateOrderBroker => Order;
}

impl BrokerTradingEndpoint for GetAccount {
    fn br_url(&self, account_id: &str) -> String {
        format!("accounts/{account_id}")
    }
}

impl BrokerTradingEndpoint for CreateOrderBroker {
    fn br_url(&self, account_id: &str) -> String {
        format!("accounts/{account_id}/orders")
    }
}

impl TradingClient {
    /// Gets the account data for this trading account.
    pub async fn get_account(&self) -> Result<Account> {
        self.execute(GetAccount).await
    }
}

impl AccountView<'_> {
    /// Create an order on behalf of this account in the Broker API.
    /// This function returns a builder, so you configure the order and call
    /// [`CreateOrderBrokerBuilder::execute`] to send it.
    pub fn create_order(
        &self,
        symbol: String,
        amount: OrderAmount,
        side: OrderSide,
    ) -> CreateOrderBrokerBuilder {
        CreateOrderBrokerBuilder(
            self,
            CreateOrderBroker {
                symbol,
                amount,
                side,
                kind: OrderType::default(),
                time_in_force: OrderTif::default(),
                extended_hours: false,
                client_order_id: None,
                order_class: OrderClass::default(),
                commission: None,
                commission_bps: None,
                source: None,
                instructions: None,
                subtag: None,
                swap_fee_bps: None,
            },
        )
    }
}
