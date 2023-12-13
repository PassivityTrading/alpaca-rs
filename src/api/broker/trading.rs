use super::*;

/// Create an order on behalf of an account in the Broker API.
#[with_builder(create_order)]
#[skip_serializing_none]
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, ClientEndpoint)]
#[endpoint(Post(json) "/orders" in AccountView -> Order)]
pub struct CreateOrderBroker {
    /// The symbol/ticker of the stock being traded.
    #[required]
    pub symbol: String,
    /// Either the quantity or the dollar amount to trade.
    #[required]
    #[serde(flatten)]
    pub amount: OrderAmount,
    #[required]
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
    pub client_order_id: Option<String>,
    // TODO explain
    pub order_class: OrderClass,
    /// The commission that the broker takes.
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub commission: Option<f64>,
    // TODO explain
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub commission_bps: Option<f64>,
    // TODO explain [no official explanation]
    pub source: Option<String>,
    // TODO explain [no official explanation]
    pub instructions: Option<String>,
    // TODO explain [no official explanation]
    pub subtag: Option<String>,
    // TODO explain [no official explanation]
    pub swap_fee_bps: Option<String>,
}
