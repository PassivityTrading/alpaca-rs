use super::*;

#[with_builder(get_open_positions)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Get "/v2/positions" in TradingClient -> Vec<OpenPosition>)]
pub struct GetOpenPositions;

#[with_builder(close_all_positions)]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Delete(query) "/v2/positions" in TradingClient)]
pub struct CloseAllPositions {
    pub cancel_orders: Option<bool>,
}

#[with_builder(get_open_position)]
#[derive(Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Get(empty) "/v2/positions/{symbol_or_asset_id}" in TradingClient -> OpenPosition)]
pub struct GetOpenPosition {
    #[required]
    pub symbol_or_asset_id: SymbolOrAssetId,
}

#[with_builder(close_position)]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Delete(query) "/v2/positions/{symbol_or_asset_id}" in TradingClient)]
pub struct ClosePosition {
    #[required]
    #[serde(skip_serializing)]
    pub symbol_or_asset_id: SymbolOrAssetId,
    pub qty: Option<f64>,
    pub percentage: Option<f64>,
}
