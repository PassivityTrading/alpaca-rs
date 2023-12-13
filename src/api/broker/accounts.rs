use super::*;

/// An account view is like a [`BrokerClient`], but scoped to a single account.
#[must_use = "An account view does not do anything unless you execute endpoints with it yourself"]
pub struct AccountView {
    data: Option<Account>,
    id: String,
    client: HttpClient<BrokerMiddleware>,
}

impl AccountView {
    // fields are private, this is the only way to init self
    pub(super) fn new(id: String, middleware: BrokerMiddleware, base_url: Url) -> Self {
        Self {
            data: None,
            id,
            client: HttpClient::new_with(middleware).with_base_url(base_url),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub async fn data(&mut self) -> Result<Account> {
        if let Some(account) = self.data.as_ref().cloned() {
            Ok(account)
        } else {
            let account = self.execute(GetAccount).await?;

            self.data = Some(account.clone());

            Ok(account)
        }
    }

    pub fn get_data(&self) -> Option<&Account> {
        self.data.as_ref()
    }

    pub async fn execute<T: ClientEndpoint<Context = Self, Error = Error>>(
        &self,
        endpoint: T,
    ) -> Result<T::Output> {
        endpoint.run(self).await
    }
}

impl HttpClientContext for AccountView {
    type Error = Error;

    fn new_request(&self, method: Method, url: &str) -> Request {
        // HACK for leading slashes in endpoint urls, the url parser does not like that when
        // joining so it just yeets out the api version from the base url (i.e.
        // api.alpaca.markets/v2 with the url /orders becomes api.alpaca.markets/orders).
        // this behavior is not very sensical but in order to allow stylish urls we can just slice
        // off the first char (i.e. the leading slash), and if others want to specify another api
        // version they could just have two (i.e. "//v2/orders").
        self.client.new_request(method, &url[1..])
    }

    async fn run_request(&self, request: Request) -> Result<Response, Self::Error> {
        self.client.run_request(request).await
    }
}

#[with_builder(get_account)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ClientEndpoint)]
#[endpoint(Get(empty) (format!("/accounts/{}", client.id())) in AccountView -> Account)]
pub struct GetAccount;

#[with_builder(get_all_accounts)]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default, ClientEndpoint)]
#[endpoint(Get(query) "/accounts" in BrokerClient -> Vec<SmallAccount>)]
pub struct GetAllAccounts {
    #[required]
    pub query: Vec<String>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub status: Option<AccountStatus>,
    #[serde(default)]
    pub sort: Sort,
    pub entities: Vec<String>,
}

#[with_builder(create_account)]
#[derive(Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Post "/accounts" in BrokerClient -> Account)]
pub struct CreateAccount {
    #[required]
    pub contact: Contact,
    #[required]
    pub identity: Identity,
    pub disclosures: Disclosures,
    pub agreements: Vec<Agreement>,
    pub documents: Vec<Document>,
    pub trusted_contact: TrustedContact,
    pub enabled_assets: Vec<String>,
}

impl CreateAccountBuilder<'_> {
    /// Add a document to this builder.
    pub fn document(mut self, document: Document) -> Self {
        self.1.documents.push(document);
        self
    }
}

// FIXME inconsistent casing? snakecase everywhere except here
#[with_builder(update_account)]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Patch(json) (format!("/accounts/{}", client.id())) in AccountView -> Account)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAccount {
    pub contact: Option<Contact>,
    pub identity: Option<Identity>,
    pub disclosures: Option<Disclosures>,
    pub trusted_contact: Option<TrustedContact>,
}
