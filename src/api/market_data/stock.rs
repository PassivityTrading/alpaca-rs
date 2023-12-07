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
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct GetHistoricalAuctions {
        pub symbols: Vec<String>,
        pub start: Option<NaiveDate>,
        pub end: Option<NaiveDate>,
        pub limit: i64,
        pub asof: Option<NaiveDateTime>,
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
            (TryInto::<usize>::try_into(self.limit).unwrap()).max(page_size),
        )]);

        if let Some(page_token) = page_token {
            builder = builder.query(&[("page_tokn", page_token)]);
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

endpoint! {
    impl GET "/v2/stocks/auctions" = GetHistoricalAuctions => HistoricalAuctions { |this, request|
        request
            .query(this)
            .query(&[("symbols", this.symbols.join(","))])
    };
}
