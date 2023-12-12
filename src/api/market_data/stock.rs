use super::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StockFeed {
    /// All US exchanges
    #[default]
    Sip,
    /// Only the Investors Exchange (IEX in short)
    #[serde(rename = "iex")]
    InvestorsExchange,
    /// Only over-the-counter (OTC) exchanges
    #[serde(rename = "otc")]
    OverTheCounter,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Get(query) "/v2/stocks/auctions" in MarketDataClient -> HistoricalAuctions)]
pub struct GetHistoricalAuctions {
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    pub symbols: Vec<String>,
    pub start: Option<Date>,
    pub end: Option<Date>,
    pub limit: Option<i64>,
    pub asof: Option<DateTime>,
    pub feed: StockFeed,
    pub currency: Option<String>,
    pub sort: Option<Sort>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, Default, ClientEndpoint)]
#[endpoint(Get(query) "/v2/stocks/bars" in MarketDataClient -> HistoricalBars)]
pub struct GetHistoricalBars {
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    pub symbols: Vec<String>,
    pub timeframe: Timeframe,
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
    pub limit: Option<i64>,
    pub adjustment: CorporateActionAdjustment,
    pub asof: Option<DateTime>,
    pub feed: StockFeed,
    pub currency: Option<String>,
    pub sort: Option<Sort>,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Get(query) "/v2/stocks/bars/latest" in MarketDataClient -> LatestBars)]
pub struct GetLatestBars {
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    pub symbols: Vec<String>,
    pub feed: StockFeed,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, ClientEndpoint)]
#[endpoint(Get(query) "/v2/stocks/meta/conditions/{tick_type}" in MarketDataClient -> HashMap<String, String>)]
pub struct ConditionCodes {
    #[serde(skip_serializing)]
    pub tick_type: TickType,
    pub tape: Tape,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, Copy, PartialEq, Eq, Hash, ClientEndpoint)]
#[endpoint(Get "/v2/stocks/meta/exchanges" in MarketDataClient -> HashMap<String, String>)]
pub struct ExchangeCodes;

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Get(query) "/v2/stocks/quotes" in MarketDataClient -> HistoricalQuotes)]
pub struct GetHistoricalQuotes {
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    pub symbols: Vec<String>,
    pub start: Option<Date>,
    pub end: Option<Date>,
    pub limit: Option<i64>,
    pub asof: Option<DateTime>,
    pub feed: StockFeed,
    pub sort: Option<Sort>,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Get(query) "/v2/stocks/quotes/latest" in MarketDataClient -> LatestQuotes)]
pub struct GetLatestQuotes {
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    pub symbols: Vec<String>,
    pub feed: StockFeed,
    pub currency: Option<String>,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Get(query) "/v2/stocks/snapshots" in MarketDataClient -> Vec<Snapshot>)]
pub struct GetSnapshots {
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    pub symbols: Vec<String>,
    pub feed: Option<StockFeed>,
    pub currency: Option<String>,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Get(query) "/v2/stocks/trades" in MarketDataClient -> HistoricalTrades)]
pub struct GetHistoricalTrades {
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    pub symbols: Vec<String>,
    pub start: Option<DateTime>,
    pub end: Option<DateTime>,
    pub limit: Option<i64>,
    pub asof: Option<DateTime>,
    pub feed: Option<StockFeed>,
    pub currency: Option<String>,
    pub sort: Option<Sort>,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Get(query) "/v2/stocks/trades/latest" in MarketDataClient -> LatestTrades)]
pub struct GetLatestTrades {
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    pub symbols: Vec<String>,
    pub feed: Option<StockFeed>,
    pub currency: Option<String>,
}
