use super::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Get "/v2/positions" in TradingClient)]
pub struct GetOpenPositions;

#[derive(Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Delete(query) "/v2/positions" in TradingClient)]
pub struct CloseAllPositions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_orders: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Get(empty) "/v2/positions/{symbol_or_asset_id}" in TradingClient)]
pub struct GetOpenPosition {
    pub symbol_or_asset_id: SymbolOrAssetId,
}

#[derive(Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Delete(query) "/v2/positions/{symbol_or_asset_id}" in TradingClient)]
pub struct ClosePosition {
    #[serde(skip_serializing)]
    pub symbol_or_asset_id: SymbolOrAssetId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}
