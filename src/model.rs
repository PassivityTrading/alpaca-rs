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
    // FIXME: nonsense
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

/// The status an order can have.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    /// The order has been received by Alpaca, and routed to exchanges for
    /// execution. This is the usual initial state of an order.
    New,
    /// The order has changed.
    Replaced,
    /// The order has been partially filled.
    PartiallyFilled,
    /// The order has been filled, and no further updates will occur for
    /// the order.
    Filled,
    /// The order is done executing for the day, and will not receive
    /// further updates until the next trading day.
    DoneForDay,
    /// The order has been canceled, and no further updates will occur for
    /// the order. This can be either due to a cancel request by the user,
    /// or the order has been canceled by the exchanges due to its
    /// time-in-force.
    Canceled,
    /// The order has expired, and no further updates will occur for the
    /// order.
    Expired,
    /// The order has been received by Alpaca, but hasn't yet been routed
    /// to the execution venue. This state only occurs on rare occasions.
    Accepted,
    /// The order has been received by Alpaca, and routed to the
    /// exchanges, but has not yet been accepted for execution. This state
    /// only occurs on rare occasions.
    PendingNew,
    /// The order has been received by exchanges, and is evaluated for
    /// pricing. This state only occurs on rare occasions.
    AcceptedForBidding,
    /// The order is waiting to be canceled. This state only occurs on
    /// rare occasions.
    PendingCancel,
    /// The order is awaiting replacement.
    PendingReplace,
    /// The order has been stopped, and a trade is guaranteed for the
    /// order, usually at a stated price or better, but has not yet
    /// occurred. This state only occurs on rare occasions.
    Stopped,
    /// The order has been rejected, and no further updates will occur for
    /// the order. This state occurs on rare occasions and may occur based
    /// on various conditions decided by the exchanges.
    Rejected,
    /// The order has been suspended, and is not eligible for trading.
    /// This state only occurs on rare occasions.
    Suspended,
    /// The order has been completed for the day (either filled or done
    /// for day), but remaining settlement calculations are still pending.
    /// This state only occurs on rare occasions.
    Calculated,
    /// The order is still being held. This may be the case for legs of
    /// bracket-style orders that are not active yet because the primary
    /// order has not filled yet.
    Held,
    /// Any other status that we have not accounted for.
    ///
    /// Note that having any such status should be considered a bug.
    #[serde(other, rename(serialize = "unknown"))]
    Unknown,
}

impl OrderStatus {
    /// Check whether the status is terminal, i.e., no more changes will
    /// occur to the associated order.
    #[inline]
    #[must_use]
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Replaced | Self::Filled | Self::Canceled | Self::Expired | Self::Rejected
        )
    }
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
    /// The order is good for the day, and it will be canceled
    /// automatically at the end of Regular Trading Hours if unfilled.
    Day,
    /// The order is only executed if the entire order quantity can
    /// be filled, otherwise the order is canceled.
    #[serde(rename = "fok")]
    FillOrKill,
    /// The order requires all or part of the order to be executed
    /// immediately. Any unfilled portion of the order is canceled.
    #[serde(rename = "ioc")]
    ImmediateOrCancel,
    /// The order is good until canceled.
    #[default]
    #[serde(rename = "gtc")]
    GoodTillCanceled,
    /// This order is eligible to execute only in the market opening
    /// auction. Any unfilled orders after the open will be canceled.
    #[serde(rename = "opg")]
    UntilMarketOpen,
    /// This order is eligible to execute only in the market closing
    /// auction. Any unfilled orders after the close will be canceled.
    #[serde(rename = "cls")]
    UntilMarketClose,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum OrderClass {
    /// Any non-bracket order (i.e., regular market, limit, or stop loss
    /// orders).
    #[default]
    Simple,
    /// A bracket order is a chain of three orders that can be used to manage your
    /// position entry and exit. It is a common use case of an
    /// one-triggers & one-cancels-other order.
    Bracket,
    /// A One-cancels-other is a set of two orders with the same side
    /// (buy/buy or sell/sell) and currently only exit order is supported.
    /// Such an order can be used to add two legs to an already filled
    /// order.
    #[serde(rename = "oco")]
    OneCancelsOther,
    /// A one-triggers-other order that can either have a take-profit or
    /// stop-loss leg set. It essentially attached a single leg to an
    /// entry order.
    #[serde(rename = "oto")]
    OneTriggersOther,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(
    tag = "bank_code_type",
    content = "bank_code",
    rename_all = "UPPERCASE"
)]
pub enum BankCode {
    ABA(String),
    BIC(String),
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BankRelationshipStatus {
    Queued,
    SentToClearing,
    Approved,
    Canceled
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct BankRelationship {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub account_id: String,
    pub status: BankRelationshipStatus,
    pub name: String,
    pub account_number: String,
    pub country: Option<String>,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub street_address: Option<String>,
}
