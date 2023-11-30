use super::*;
use chrono::{DateTime, Utc};

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

impl BrokerEndpoint for CreateAccount {}
impl BrokerEndpoint for GetAllAccounts {}

endpoint! {
    impl GET "/accounts" = GetAllAccounts => Vec<SmallAccount> { |this, request| request.query(this).query(&[("query", this.query.join(" "))]) };
    impl POST "/accounts" = CreateAccount => Account { |this, request| request.json(this) };
}
