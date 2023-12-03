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

impl BrokerTradingEndpoint for UpdateAccount {
    fn br_url(&self, account_id: &str) -> String {
        format!("accounts/{account_id}")
    }
}

with_builder! { |account|
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateBankRelationship {
        pub name: String,
        pub bank_code: BankCode,
        pub bank_code_type: BankCodeType,
        pub account_number: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub state_province: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub street_address: Option<String>
    }
}

impl BrokerTradingEndpoint for CreateBankRelationship {
    fn br_url(&self, account_id: &str) -> String {
        format!("accounts/{account_id}/recipient_banks")
    }
}

with_builder! { |account|
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateAchRelationship {
        pub account_owner_name: String,
        pub bank_account_type: String,
        pub bank_account_number: String,
        pub bank_routing_number: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub nickname: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub processor_token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub instant: Option<bool>,
    }
}

impl BrokerTradingEndpoint for CreateAchRelationship {
    fn br_url(&self, account_id: &str) -> String {
        format!("accounts/{account_id}/ach_relationships")
    }
}

with_builder! { |account|
    #[serde_as]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateTransfer {
        pub transfer_type: TransferType,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub relationship_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bank_id: Option<String>,
        #[serde_as(as = "DisplayFromStr")]
        pub amount: f64,
        pub direction: Direction,
        pub timing: Timing,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub additional_information: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fee_payment_method: Option<String>,
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
    impl POST "/ach_relationships" = CreateAchRelationship => AchRelationship { |this, request| request.json(this) };
    impl POST "/transfer" = CreateTransfer => Transfer { |this, request| request.json(this) };
}
