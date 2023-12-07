//! A module providing pagination capabilities.
use super::*;

pub trait PaginationEndpoint {
    type Item;
    type Response: Default;

    fn configure(
        &self,
        builder: reqwest::RequestBuilder,
        page_size: usize,
        page_token: Option<String>,
    ) -> reqwest::RequestBuilder;

    fn next_page_token(&self, response: &Self::Response) -> Option<String>;

    fn deserialize(
        response: reqwest::Response,
    ) -> impl Future<Output = Result<Self::Response, Error>> + 'static;
}

#[doc(hidden)]
pub trait Paginatable<E: PaginationEndpoint>: Sized {
    fn run_request(
        &self,
        endpoint: &E,
        page_size: usize,
        page_token: Option<String>,
    ) -> impl Future<Output = Result<<E as PaginationEndpoint>::Response, Error>>
    where
        E::Item: DeserializeOwned;
}

pub struct PaginationClient<'a, E: PaginationEndpoint, P> {
    pub page: E::Response,
    client: &'a P,
    config: E,
    page_size: usize,
    page_token: Option<String>,
}

impl<'a, E: PaginationEndpoint, P> PaginationClient<'a, E, P> {
    #[doc(hidden)]
    pub fn new(client: &'a P, config: E, page_size: usize) -> Self {
        Self {
            client,
            page: E::Response::default(),
            config,
            page_size,
            page_token: None,
        }
    }
}

impl<'a, T: Identifiable, E: PaginationEndpoint<Item = T>, P: Paginatable<E>>
    PaginationClient<'a, E, P>
{
    pub async fn next(&mut self) -> Result<(), Error>
    where
        T: DeserializeOwned,
    {
        self.page = self
            .client
            .run_request(&self.config, self.page_size, self.page_token.take())
            .await?;
        self.page_token = self.config.next_page_token(&self.page);

        Ok(())
    }

    pub fn set_page_size(&mut self, page_size: usize) {
        self.page_size = page_size;
    }
}

impl<E: PaginationEndpoint + TradingEndpoint + Endpoint> Paginatable<E> for TradingClient {
    fn run_request(
        &self,
        endpoint: &E,
        page_size: usize,
        page_token: Option<String>,
    ) -> impl Future<Output = Result<<E as PaginationEndpoint>::Response, Error>>
    where
        <E as PaginationEndpoint>::Item: DeserializeOwned,
    {
        // why does this need to be out of the async? I don't know!
        let requ = <E as PaginationEndpoint>::configure(
            endpoint,
            self.reqwest.request(
                endpoint.method(),
                endpoint
                    .base_url(self)
                    .join(endpoint.url().as_ref())
                    .unwrap(),
            ),
            page_size,
            page_token,
        );

        async move { <E as PaginationEndpoint>::deserialize(requ.send().await?).await }
    }
}

// FIXME code duplication - why is this the same code?
impl<E: PaginationEndpoint + BrokerEndpoint + Endpoint> Paginatable<E> for BrokerClient {
    fn run_request(
        &self,
        endpoint: &E,
        page_size: usize,
        page_token: Option<String>,
    ) -> impl Future<Output = Result<<E as PaginationEndpoint>::Response, Error>>
    where
        <E as PaginationEndpoint>::Item: DeserializeOwned,
    {
        // why does this need to be out of the async? I don't know!
        let requ = <E as PaginationEndpoint>::configure(
            endpoint,
            self.reqwest.request(
                endpoint.method(),
                endpoint
                    .base_url(self)
                    .join(endpoint.url().as_ref())
                    .unwrap(),
            ),
            page_size,
            page_token,
        );

        async move { <E as PaginationEndpoint>::deserialize(requ.send().await?).await }
    }
}

impl TradingClient {
    pub fn paginate<E: PaginationEndpoint + TradingEndpoint>(
        &self,
        config: E,
        page_size: usize,
    ) -> PaginationClient<E, Self> {
        PaginationClient::new(self, config, page_size)
    }
}

impl BrokerClient {
    pub fn paginate<E: PaginationEndpoint + BrokerEndpoint>(
        &self,
        config: E,
        page_size: usize,
    ) -> PaginationClient<E, Self> {
        PaginationClient::new(self, config, page_size)
    }
}
