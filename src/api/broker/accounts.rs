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
        self.client.new_request(method, url)
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
#[derive(Debug, Clone, Serialize, Deserialize, Default, ClientEndpoint)]
#[endpoint(Get(query) "/accounts" in BrokerClient -> Vec<SmallAccount>)]
pub struct GetAllAccounts {
    #[required]
    pub query: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[required]
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
#[derive(Default, Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Patch(json) (format!("/accounts/{}", client.id())) in AccountView -> Account)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosures: Option<Disclosures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_contact: Option<TrustedContact>,
}
