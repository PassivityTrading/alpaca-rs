//! (TODO) A module providing pagination capabilities.
// use super::*;

use acril::Service;

pub trait PaginationEndpoint: Service + Sized {
    type Output;
    type PageToken;

    fn next_page_token(output: &Self::Output) -> Self::PageToken;

    async fn next(
        &self,
        previous: Option<Self::PageToken>,
        client: &Self::Context,
        pagination: &PaginationContext<Self>,
    ) -> Result<Self::Output, Self::Error>;

    fn paginate<'a>(
        self,
        page_size: usize,
        client: &'a Self::Context,
    ) -> PaginationContext<'a, Self> {
        PaginationContext {
            client,
            endpoint: self,
            page_size,
            last_page_token: None,
        }
    }
}

pub struct PaginationContext<'a, E: PaginationEndpoint> {
    client: &'a E::Context,
    endpoint: E,
    pub page_size: usize,
    last_page_token: Option<E::PageToken>,
}

impl<E: PaginationEndpoint> PaginationContext<'_, E> {
    pub async fn next(&mut self) -> Result<E::Output, E::Error> {
        let last_page_token = self.last_page_token.take();
        let output = self
            .endpoint
            .next(last_page_token, self.client, self)
            .await?;

        self.last_page_token = Some(E::next_page_token(&output));

        Ok(output)
    }
}
