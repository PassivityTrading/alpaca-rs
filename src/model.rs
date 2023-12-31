//! This module defines all the Alpaca APIs' data types.
use super::*;

use std::collections::HashMap;
use std::fmt::Display;

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};

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

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Account {
    pub id: String,
    pub account_number: String,
    pub status: AccountStatus,
    pub crypto_status: AccountStatus,
    pub currency: String,
    pub created_at: DateTime,
    pub last_equity: String,
    #[serde_as(as = "DisplayFromStr")]
    pub portfolio_value: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub cash: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub buying_power: f64,
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

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Default)]
#[serde(rename_all = "snake_case", tag = "type")]
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
    pub id: String,
    pub symbol: String,
    pub status: OrderStatus,
    pub side: OrderSide,
    #[serde(rename = "type", flatten)]
    pub kind: OrderType,
}

#[derive(
    Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Default, derive_more::Display,
)]
#[serde(rename_all = "lowercase")]
pub enum OrderTif {
    /// The order is good for the day, and it will be canceled
    /// automatically at the end of Regular Trading Hours if unfilled.
    Day,
    /// The order is only executed if the entire order quantity can
    /// be filled, otherwise the order is canceled.
    #[serde(rename = "fok")]
    #[display(fmt = "fill or kill")]
    FillOrKill,
    /// The order requires all or part of the order to be executed
    /// immediately. Any unfilled portion of the order is canceled.
    #[serde(rename = "ioc")]
    #[display(fmt = "immediate or cancel")]
    ImmediateOrCancel,
    /// The order is good until canceled.
    #[default]
    #[serde(rename = "gtc")]
    #[display(fmt = "good till canceled")]
    GoodTillCanceled,
    /// This order is eligible to execute only in the market opening
    /// auction. Any unfilled orders after the open will be canceled.
    #[serde(rename = "opg")]
    #[display(fmt = "until market open")]
    UntilMarketOpen,
    /// This order is eligible to execute only in the market closing
    /// auction. Any unfilled orders after the close will be canceled.
    #[serde(rename = "cls")]
    #[display(fmt = "until market close")]
    UntilMarketClose,
}

#[derive(
    Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Default, derive_more::Display,
)]
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
    #[display(fmt = "one cancels other")]
    OneCancelsOther,
    /// A one-triggers-other order that can either have a take-profit or
    /// stop-loss leg set. It essentially attached a single leg to an
    /// entry order.
    #[serde(rename = "oto")]
    #[display(fmt = "one triggers other")]
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
    Canceled,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct BankRelationship {
    pub id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
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

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AchRelationshipStatus {
    Queued,
    Approved,
    Pending,
    CancelRequested,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BankAccountType {
    Checking,
    Savings,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct AchRelationship {
    pub id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub account_id: String,
    pub status: AchRelationshipStatus,
    pub account_owner_name: String,
    pub bank_account_type: Option<BankAccountType>,
    pub bank_account_number: Option<String>,
    pub bank_routing_number: Option<String>,
    pub nickname: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferType {
    Ach,
    Wire,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Direction {
    Incoming,
    Outgoing,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Timing {
    #[default]
    Immediate,
    NextDay,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferStatus {
    Queued,
    ApprovalPending,
    Pending,
    SentToClearing,
    Rejected,
    Canceled,
    Approved,
    Complete,
    Returned,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Transfer {
    pub id: String,
    pub relationship_id: Option<String>,
    pub bank_id: Option<String>,
    pub account_id: String,
    #[serde(rename = "type")]
    pub kind: TransferType,
    pub status: TransferStatus,
    pub reason: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: f64,
    pub direction: Direction,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub expires_at: Option<DateTime>,
    pub additional_information: Option<String>,
    pub hold_until: Option<DateTime>,
    pub instant_amount: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BankCodeType {
    Aba,
    Bic,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    #[default]
    Long,
    Short,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AssetClass {
    UsEquity,
    Crypto,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct OpenPosition {
    pub asset_id: String,
    pub symbol: String,
    pub exchange: String,
    pub asset_class: String,
    pub asset_marginable: Option<bool>,
    #[serde_as(as = "DisplayFromStr")]
    pub avg_entry_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub qty: i64,
    pub side: Side,
    #[serde_as(as = "DisplayFromStr")]
    pub market_value: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub cost_basis: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub unrealized_pl: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub unrealized_plpc: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub unrealized_intraday_pl: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub unrealized_intraday_plpc: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub current_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub lastday_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub change_today: f64,
    pub swap_rate: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SymbolOrAssetId {
    SymbolId(String),
    AssetId(String),
}

impl Display for SymbolOrAssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolOrAssetId::SymbolId(symbol) => write!(f, "{symbol}"),
            SymbolOrAssetId::AssetId(asset_id) => write!(f, "{asset_id}"),
        }
    }
}

/// Order amount (number of shares or dollar amount).
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, derive_more::Display)]
pub enum OrderAmount {
    #[serde(rename = "qty")]
    #[display(fmt = "{_0} shares of")]
    /// Number of shares.
    Quantity(#[serde_as(as = "DisplayFromStr")] i64),
    #[serde(rename = "notional")]
    #[display(fmt = "${_0} worth of")]
    /// Notional amount is the amount of stock in the currency of the account.
    Notional(#[serde_as(as = "DisplayFromStr")] f64),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct HistoricalAuctions {
    pub next_page_token: Option<String>,
    pub currency: Option<String>,
    pub auctions: Vec<HistoricalAuction>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SingleAuction {
    #[serde(rename = "t")]
    pub timestamp: DateTime,
    #[serde(rename = "x")]
    pub exchange_code: String,
    #[serde(rename = "p")]
    pub price: f64,
    #[serde(rename = "s")]
    pub size: Option<i64>,
    #[serde(rename = "c")]
    pub condition: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct HistoricalAuction {
    #[serde(rename = "d")]
    pub date: Date,
    #[serde(rename = "o")]
    pub opening: Vec<SingleAuction>,
    #[serde(rename = "c")]
    pub closing: Vec<SingleAuction>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Timeframe {
    Minutes(u8),
    Hours(u8),
    #[default]
    Day,
    Week,
    Months(u8),
}

use serde_with::{DeserializeAs, SerializeAs};

impl Serialize for Timeframe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        DisplayFromStr::serialize_as(self, serializer)
    }
}

impl<'de> Deserialize<'de> for Timeframe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        DisplayFromStr::deserialize_as(deserializer)
    }
}

impl Display for Timeframe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minutes(minutes) => write!(f, "{minutes}T"),
            Self::Hours(hours) => write!(f, "{hours}H"),
            Self::Day => write!(f, "1D"),
            Self::Week => write!(f, "1W"),
            Self::Months(months) => write!(f, "{months}M"),
        }
    }
}

impl std::str::FromStr for Timeframe {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn number(s: &str) -> Result<u8, &'static str> {
            if s.chars().nth(1).ok_or("no characters")?.is_ascii_digit() {
                &s[..=1]
            } else {
                &s[..1]
            }
            .parse::<u8>()
            .map_err(|_| "invalid number")
        }

        Ok(match s.chars().last().ok_or("no characters")? {
            // Min / T
            'T' | 'n' => Self::Minutes(number(s)?),
            // Hour / H
            'H' | 'r' => Self::Hours(number(s)?),
            // Day / D
            'D' | 'y' => Self::Day,
            // Week / W
            'W' | 'k' => Self::Week,
            // Month / M
            'M' | 'h' => Self::Months(number(s)?),
            _ => return Err("invalid ending character"),
        })
    }
}

// TODO explain
#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum CorporateActionAdjustment {
    Raw,
    Split,
    Dividend,
    #[default]
    All,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct HistoricalBars {
    pub bars: HashMap<String, Vec<HistoricalBar>>,
    // required here, type is String | null
    pub next_page_token: Option<String>,
    // not required here, type is String or undefined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct HistoricalBar {
    #[serde(rename = "t")]
    pub timestamp: DateTime,
    #[serde(rename = "o")]
    pub opening_price: f64,
    #[serde(rename = "h")]
    pub high_price: f64,
    #[serde(rename = "l")]
    pub low_price: f64,
    #[serde(rename = "c")]
    pub closing_price: f64,
    #[serde(rename = "v")]
    pub volume: i64,
    #[serde(rename = "n")]
    pub trade_count: i64,
    #[serde(rename = "vw")]
    pub avg_vol_weighted: f64,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LatestBars {
    pub bars: Vec<HistoricalBar>,
    pub currency: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
pub enum TickType {
    #[display(fmt = "trade")]
    Trade,
    #[display(fmt = "quote")]
    Quote,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum Tape {
    A,
    B,
    C,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuoteZone {
    /// NY stock exchange
    #[serde(rename = "A")]
    NYSE,
    /// NYSE Arca, Bats, IEX and other regional exchanges
    #[serde(rename = "B")]
    Regional,
    /// The NASDAQ exchange
    #[serde(rename = "C")]
    Nasdaq,
    // TODO explain
    /// OTC
    #[serde(rename = "O")]
    Otc,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Quote {
    #[serde(rename = "t")]
    pub timestamp: DateTime,
    #[serde(rename = "bx")]
    pub bid_exchange: String,
    #[serde(rename = "bp")]
    pub bid_price: f64,
    #[serde(rename = "bs")]
    pub bid_size: u32,
    #[serde(rename = "ax")]
    pub ask_exchange: String,
    #[serde(rename = "ap")]
    pub ask_price: f64,
    #[serde(rename = "as")]
    pub ask_size: u32,
    #[serde(rename = "c")]
    pub condition_flags: Vec<String>,
    // TODO(doc): is the name `zone` correct for `z`?
    #[serde(rename = "z")]
    pub zone: QuoteZone,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct HistoricalQuotes {
    pub quotes: Vec<Quote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    pub next_page_token: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LatestQuotes {
    pub quotes: Vec<Quote>,
    pub currency: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Clock {
    pub timestamp: DateTime,
    pub is_open: bool,
    pub next_open: DateTime,
    pub next_close: DateTime,
}

#[derive(Default, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum DateType {
    #[default]
    Trading,
    Settlement,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Calendar(pub Vec<CalendarDay>);

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CalendarDay {
    #[serde_as(as = "DisplayFromStr")]
    pub date: Date,
    #[serde_as(as = "DisplayFromStr")]
    pub open: NaiveTime,
    #[serde_as(as = "DisplayFromStr")]
    pub close: NaiveTime,
    #[serde_as(as = "DisplayFromStr")]
    pub settlement_date: Date,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HistoricalTrade {
    #[serde(rename = "t")]
    pub timestamp: DateTime,
    #[serde(rename = "x")]
    pub exchange_code: String,
    #[serde(rename = "p")]
    pub price: f64,
    #[serde(rename = "s")]
    pub size: u32,
    #[serde(rename = "i")]
    pub trade_id: i64,
    #[serde(rename = "c")]
    pub condition_flags: Vec<String>,
    #[serde(rename = "z")]
    pub tape: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub daily_bar: HistoricalBar,
    pub latest_quote: Quote,
    pub latest_trade: HistoricalTrade,
    pub minute_bar: HistoricalBar,
    #[serde(rename = "prev_daily_bar")]
    pub previous_daily_bar: HistoricalBar,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct HistoricalTrades {
    pub trades: Vec<HistoricalTrade>,
    pub next_page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LatestTrades {
    pub trades: Vec<HistoricalTrade>,
    pub currency: Option<String>,
}
