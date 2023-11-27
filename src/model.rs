use super::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Onboarding,
    Submitted,
    Resubmitted,
    SubmissionFailed,
    ActionRequired,
    Edited,
    AccountUpdated,
    ApprovalPending,
    ReapprovalPending,
    SignedUp,
    KycSubmitted,
    Limited,
    AmlReview,
    Approved,
    Rejected,
    Disabled,
    DisablePending,
    AccountClosed,
    PaperOnly,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Sort {
    #[serde(rename = "asc")]
    Ascending,
    #[default]
    #[serde(rename = "desc")]
    Descending
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    Trading,
    Custodial,
    DonorAdvised
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Account {
    pub id: String,
    pub account_number: String,
    pub account_type: AccountType,
    pub status: Status,
    pub crypto_status: Status,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub last_equity: String,
    pub enabled_assets: Vec<String>,
    pub contact: Contact,
    pub identity: Identity,
    pub disclosures: Disclosures,
    // nothendev: WHY????
    pub documents: Vec<Vec<Document>>,
    pub agreements: Vec<Agreement>,
    pub trusted_contact: TrustedContact,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    pub email_address: String,
    pub phone_number: String,
    pub street_address: Vec<String>,
    pub unit: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    pub given_name: String,
    pub family_name: String,
    pub date_of_birth: String,
    pub tax_id: String,
    pub tax_id_type: String,
    pub country_of_citizenship: String,
    pub country_of_birth: String,
    pub country_of_tax_residence: String,
    pub funding_source: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disclosures {
    pub is_control_person: bool,
    pub is_affiliated_exchange_or_finra: bool,
    pub is_politically_exposed: bool,
    pub immediate_family_exposed: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub document_id: String,
    pub document_type: String,
    pub created_at: String,
    pub mime_type: String,
    pub content: String,
    pub document_sub_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Agreement {
    pub agreement: String,
    pub signed_at: String,
    pub ip_address: String,
    pub revision: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrustedContact {
    pub given_name: String,
    pub family_name: String,
    pub email_address: String,
}

pub mod broker;
pub mod market_data;
pub mod trading;
