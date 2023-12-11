use super::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GetOpenPositions;

with_builder! { |account|
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CloseAllPositions {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cancel_orders: Option<bool>
    }
}

with_builder! { |account|
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GetOpenPosition {
        pub symbol_or_asset_id: SymbolOrAssetId,
    }
}

with_builder! { |account|
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ClosePosition {
        pub symbol_or_asset_id: SymbolOrAssetId,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub qty: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub percentage: Option<f64>,
    }
}

endpoint! {
    impl GET "/v2/positions" = GetOpenPositions => Vec<OpenPosition>
    | broker
    | trading
    | account;
    impl GET (|Self { symbol_or_asset_id }| format!("/v2/positions/{symbol_or_asset_id}")) = GetOpenPosition => OpenPosition
    | trading
    | account;
    impl DELETE "/v2/positions" = CloseAllPositions { |this, request| request.query(this) }
    | trading
    | account;
    impl DELETE
        (|Self { symbol_or_asset_id, .. }| format!("/v2/positions/{symbol_or_asset_id}"))
        = ClosePosition { |this, request|
            request.query(&[("qty", this.qty), ("percentage", this.percentage)])
        }
    | trading
    | account
}
