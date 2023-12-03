use super::*;

with_builder! { |broker|
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct GetAllAccounts {
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
}

with_builder! { |broker|
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateAccount {
        pub contact: Contact,
        pub identity: Identity,
        pub disclosures: Disclosures,
        pub agreements: Vec<Agreement>,
        pub documents: Vec<Document>,
        pub trusted_contact: TrustedContact,
        pub enabled_assets: Vec<String>,
    }
}

impl CreateAccountBuilder<'_> {
    /// Add a document to this builder.
    pub fn document(mut self, document: Document) -> Self {
        self.1.documents.push(document);
        self
    }
}

with_builder! { |account|
    // FIXME inconsistent casing? snakecase everywhere except here
    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UpdateAccount {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contact: Option<Contact>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub identity: Option<Identity>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub disclosures: Option<Disclosures>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trusted_contact: Option<TrustedContact>
    }
}

impl BrokerClient {
    pub fn create_account(&self, contact: Contact, identity: Identity) -> CreateAccountBuilder {
        CreateAccountBuilder(
            self,
            CreateAccount {
                contact,
                identity,
                disclosures: Default::default(),
                agreements: vec![],
                documents: vec![],
                trusted_contact: Default::default(),
                enabled_assets: vec![],
            },
        )
    }

    pub fn get_all_accounts(&self) -> GetAllAccountsBuilder {
        GetAllAccountsBuilder(self, GetAllAccounts::default())
    }
}

impl AccountView<'_> {
    pub fn update(&self) -> UpdateAccountBuilder {
        UpdateAccountBuilder(self, UpdateAccount::default())
    }
}

endpoint! {
    impl GET "/accounts" = GetAllAccounts => Vec<SmallAccount> { |this, request| request.query(this).query(&[("query", this.query.join(" "))]) };
    impl POST "/accounts" = CreateAccount => Account { |this, request| request.json(this) };
    impl PATCH "/accounts" = UpdateAccount => Account { |this, request| request.json(this) }
    | account (|_, account_id| format!("/accounts/{account_id}"));
}
