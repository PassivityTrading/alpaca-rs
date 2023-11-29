use super::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Default, Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountStatus {
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
    #[default]
    Active,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Sort {
    #[serde(rename = "asc")]
    Ascending,
    #[default]
    #[serde(rename = "desc")]
    Descending,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    Trading,
    Custodial,
    DonorAdvised,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Account {
    pub id: String,
    pub account_number: String,
    pub account_type: AccountType,
    pub status: AccountStatus,
    pub crypto_status: AccountStatus,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmallAccount {
    pub id: String,
    pub account_number: String,
    pub status: AccountStatus,
    pub crypto_status: AccountStatus,
    pub currency: String,
    pub last_equity: String,
    pub created_at: String,
    pub account_type: String,
    pub enabled_assets: Vec<String>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderStatus {
    #[default]
    Open,
    Closed,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Default)]
pub enum OrderType {
    #[default]
    Market,
    Limit {
        #[serde_as(as = "DisplayFromStr")]
        limit_price: f64,
    },
    Stop {
        #[serde_as(as = "DisplayFromStr")]
        stop_price: f64,
    },
    StopLimit {
        #[serde_as(as = "DisplayFromStr")]
        stop_price: f64,
        #[serde_as(as = "DisplayFromStr")]
        limit_price: f64,
    },
    #[serde(untagged)]
    TrailingStop(TrailingStop),
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum TrailingStop {
    #[serde(rename = "trail_price")]
    Price(#[serde_as(as = "DisplayFromStr")] f64),
    #[serde(rename = "trail_percent")]
    Percent(#[serde_as(as = "DisplayFromStr")] f64),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub symbol: String,
    pub status: OrderStatus,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub kind: OrderType,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum OrderTif {
    #[default]
    Day,
    #[serde(rename = "gtc")]
    GoodTillCancelled,
    #[serde(rename = "opg")]
    Opg,
    #[serde(rename = "cls")]
    Cls,
    #[serde(rename = "ioc")]
    ImmediateOrCancel,
    #[serde(rename = "fok")]
    FillOrKill,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum OrderClass {
    Simple,
    #[default]
    Bracket,
    Oco,
    Oto,
}

pub mod broker;
pub mod market_data;
pub mod trading;
