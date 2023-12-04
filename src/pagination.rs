//! A module providing pagination capabilities.
use super::*;

pub trait PaginationEndpoint {
    type Item;

    fn configure(&self, page_size: usize, page_token: Option<String>) -> reqwest::RequestBuilder;

    fn deserialize(
        response: reqwest::Response,
    ) -> impl Future<Output = Result<Vec<Self::Item>, Error>> + 'static;
}

#[doc(hidden)]
pub trait Paginatable<E: PaginationEndpoint>: Sized {
    fn run_request(
        &self,
        req: reqwest::RequestBuilder,
    ) -> impl Future<Output = Result<Vec<<E as PaginationEndpoint>::Item>, Error>> + '_
    where
        E::Item: DeserializeOwned;
}

pub struct PaginationClient<'a, E: PaginationEndpoint, P> {
    pub page: Vec<E::Item>,
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
            page: vec![],
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
            .run_request(
                self.config
                    .configure(self.page_size, self.page_token.take()),
            )
            .await?;
        self.page_token = self.page.last().map(Identifiable::id);

        Ok(())
    }

    pub fn set_page_size(&mut self, page_size: usize) {
        self.page_size = page_size;
    }
}

impl<E: PaginationEndpoint + TradingEndpoint> Paginatable<E> for TradingClient {
    fn run_request(
        &self,
        req: reqwest::RequestBuilder,
    ) -> impl Future<Output = Result<Vec<<E as PaginationEndpoint>::Item>, Error>> + '_
    where
        E::Item: DeserializeOwned,
    {
        async move {
            Ok(self
                .reqwest
                .execute(req.headers(self.auth_headers()).build()?)
                .await?
                .error_for_status()?
                .json()
                .await?)
        }
    }
}

impl<E: PaginationEndpoint + BrokerEndpoint> Paginatable<E> for BrokerClient {
    fn run_request(
        &self,
        req: reqwest::RequestBuilder,
    ) -> impl Future<Output = Result<Vec<<E as PaginationEndpoint>::Item>, Error>> + '_
    where
        E::Item: DeserializeOwned,
    {
        async move {
            Ok(self
                .reqwest
                .execute(
                    req.header(AUTHORIZATION, self.authorization_header())
                        .build()?,
                )
                .await?
                .error_for_status()?
                .json()
                .await?)
        }
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
