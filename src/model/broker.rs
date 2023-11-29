use super::*;
use chrono::{DateTime, Utc};
use reqwest::Method;

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

impl Endpoint for GetAllAccounts {
    type Result = Vec<SmallAccount>;

    fn url(&self) -> &'static str {
        "accounts"
    }

    fn method(&self) -> reqwest::Method {
        Method::GET
    }

    fn configure(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request
            .query(self)
            .query(&[("query", self.query.join(" "))])
    }

    fn deserialize(
        response: reqwest::Response,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Result>> + 'static>> {
        json_self(response)
    }
}

impl BrokerEndpoint for GetAllAccounts {}

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

impl Endpoint for CreateAccount {
    type Result = Account;

    fn configure(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request.json(self)
    }

    fn method(&self) -> reqwest::Method {
        Method::POST
    }

    fn url(&self) -> &'static str {
        "accounts"
    }

    fn deserialize(
        response: reqwest::Response,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Result>> + 'static>> {
        json_self(response)
    }
}

impl BrokerEndpoint for CreateAccount {}
