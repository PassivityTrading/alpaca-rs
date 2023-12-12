use super::*;

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
    #[serde(flatten)]
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

impl std::fmt::Display for CreateOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            symbol,
            side,
            amount,
            kind,
            time_in_force,
            extended_hours,
            order_class,
            ..
        } = self;

        let allow_ext_hours = extended_hours
            .then_some(", allowed to process in extended hours")
            .unwrap_or("");

        // not adding anything after amount because it fmts it nicely in the display impl of
        // OrderAmount
        write!(f, "Creating a {side} {kind:?} order, for {amount} ${symbol} ({order_class}, {time_in_force}{allow_ext_hours})")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, ClientEndpoint)]
#[endpoint(Delete(empty, empty) "/orders/{order_id}" in TradingClient)]
pub struct CancelOrder {
    pub order_id: String,
}
