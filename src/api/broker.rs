use super::*;
use chrono::{DateTime, Utc};

with_builder! { |broker|
    #[derive(Debug, Clone, Serialize, Deserialize)]
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
        pub contact: Option<Contact>,
        pub identity: Option<Identity>,
        pub disclosures: Option<Disclosures>,
        pub trusted_contact: Option<TrustedContact>
    }
}

with_builder! { |account|
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateBankRelationship {
        pub name: String,
        pub bank_code: BankCode,
        pub account_number: String,
        pub country: Option<String>,
        pub state_province: Option<String>,
        pub postal_code: Option<String>,
        pub city: Option<String>,
        pub street_address: Option<String>
    }
}

impl BrokerTradingEndpoint for UpdateAccount {
    fn br_url(&self, account_id: &str) -> String {
        format!("accounts/{account_id}")
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
}
impl AccountView<'_> {
    pub fn update(&self) -> UpdateAccountBuilder {
        UpdateAccountBuilder(self, UpdateAccount::default())
    }
}

endpoint! {
    impl GET "/accounts" = GetAllAccounts => Vec<SmallAccount> { |this, request| request.query(this).query(&[("query", this.query.join(" "))]) };
    impl POST "/accounts" = CreateAccount => Account { |this, request| request.json(this) };
    impl PATCH "/accounts" = UpdateAccount => Account { |this, request| request.json(this) };
    impl POST "/recipient_banks" = CreateBankRelationship => BankRelationship { |this, request| request.json(this) };
}
