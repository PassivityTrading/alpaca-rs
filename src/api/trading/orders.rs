use super::*;

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

endpoint! {
    impl POST "/orders" = CreateOrder => Order { |this, request| request.json(this) };
}
