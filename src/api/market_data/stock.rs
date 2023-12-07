use crate::pagination::PaginationEndpoint;

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

with_builder! { |market_data|
    #[skip_serializing_none]
    #[serde_as]
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct GetHistoricalAuctions {
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
        pub symbols: Vec<String>,
        pub start: Option<Date>,
        pub end: Option<Date>,
        pub limit: Option<i64>,
        pub asof: Option<DateTime>,
        pub feed: StockFeed,
        pub currency: Option<String>,
        pub sort: Option<Sort>
    }
}

impl PaginationEndpoint for GetHistoricalAuctions {
    type Item = HistoricalAuction;
    type Response = HistoricalAuctions;

    fn configure(
        &self,
        request: reqwest::RequestBuilder,
        page_size: usize,
        page_token: Option<String>,
    ) -> reqwest::RequestBuilder {
        let mut builder = Endpoint::configure(self, request).query(&[(
            "limit",
            self.limit
                .and_then(|x| TryInto::<usize>::try_into(x).ok())
                .map(|x| x.max(page_size)),
        )]);

        if let Some(page_token) = page_token {
            builder = builder.query(&[("page_token", page_token)]);
        }

        builder
    }

    fn next_page_token(&self, response: &Self::Response) -> Option<String> {
        response.next_page_token.clone()
    }

    fn deserialize(
        response: reqwest::Response,
    ) -> impl Future<Output = Result<Self::Response, Error>> + 'static {
        <Self as Endpoint>::deserialize(response)
    }
}

with_builder! { |market_data|
    #[serde_as]
    #[skip_serializing_none]
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct GetHistoricalBars {
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
        pub symbols: Vec<String>,
        pub timeframe: Timeframe,
        pub start: Option<Date>,
        pub end: Option<Date>,
        pub limit: Option<i64>,
        pub adjustment: CorporateActionAdjustment,
        pub asof: Option<DateTime>,
        pub feed: StockFeed,
        pub currency: Option<String>,
        pub sort: Option<Sort>
    }
}

// FIXME code duplication
impl PaginationEndpoint for GetHistoricalBars {
    type Item = HistoricalBar;
    type Response = HistoricalBars;

    fn configure(
        &self,
        request: reqwest::RequestBuilder,
        page_size: usize,
        page_token: Option<String>,
    ) -> reqwest::RequestBuilder {
        let mut builder = Endpoint::configure(self, request).query(&[(
            "limit",
            self.limit
                .and_then(|x| TryInto::<usize>::try_into(x).ok())
                .map(|x| x.max(page_size)),
        )]);

        if let Some(page_token) = page_token {
            builder = builder.query(&[("page_token", page_token)]);
        }

        builder
    }

    fn next_page_token(&self, response: &Self::Response) -> Option<String> {
        response.next_page_token.clone()
    }

    fn deserialize(
        response: reqwest::Response,
    ) -> impl Future<Output = Result<Self::Response, Error>> + 'static {
        <Self as Endpoint>::deserialize(response)
    }
}

with_builder! { |market_data|
    #[serde_as]
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct GetLatestBars {
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
        pub symbols: Vec<String>,
        pub feed: StockFeed,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub currency: Option<String>
    }
}

with_builder! { |market_data|
    #[skip_serializing_none]
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ConditionCodes {
        pub tick_type: TickType,
        pub tape: Tape,
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, Copy, PartialEq, Eq, Hash)]
pub struct ExchangeCodes;

with_builder! { |market_data|
    #[skip_serializing_none]
    #[serde_as]
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct GetHistoricalQuotes {
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
        pub symbols: Vec<String>,
        pub start: Option<Date>,
        pub end: Option<Date>,
        pub limit: Option<i64>,
        pub asof: Option<DateTime>,
        pub feed: StockFeed,
        pub sort: Option<Sort>
    }
}

with_builder! { |market_data|
    #[skip_serializing_none]
    #[serde_as]
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct GetLatestQuotes {
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
        pub symbols: Vec<String>,
        pub feed: StockFeed,
        pub currency: Option<String>
    }
}

endpoint! {
    impl GET "/v2/stocks/auctions" = GetHistoricalAuctions => HistoricalAuctions { |this, request| request.query(this) };
    impl GET "/v2/stocks/bars" = GetHistoricalBars => HistoricalBars { |this, request| request.query(this) };
    impl GET "/v2/stocks/bars/latest" = GetLatestBars => LatestBars { |this, request| request.query(this) };
    impl GET (|Self { tick_type, .. }| format!("/v2/stocks/meta/conditions/{tick_type}")) = ConditionCodes => HashMap<String, String> { |this, request| request.query(&[("tape", this.tape)]) };
    impl GET "/v2/stocks/meta/exchanges" = ExchangeCodes => HashMap<String, String>
        | market_data;
    impl GET "/v2/stocks/quotes" = GetHistoricalQuotes => HistoricalQuotes { |this, request| request.query(this) };
    impl GET "/v2/stocks/quotes/latest" = GetLatestQuotes => LatestQuotes { |this, request| request.query(this) };
}
