use std::ops::RangeBounds;

use super::*;

mod assets;
mod orders;
mod positions;

pub use assets::*;
pub use orders::*;
pub use positions::*;

/// Get account details.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GetAccount;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GetClock;

with_builder! { |trading|
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct GetCalendar {
        pub start: Option<Date>,
        pub end: Option<Date>,
        pub date_type: DateType
    }
}

impl TradingClient {
    /// Gets the account data for this trading account.
    pub async fn get_account(&self) -> Result<Account> {
        self.execute(GetAccount).await
    }
    pub async fn get_clock(&self) -> Result<Clock> {
        self.execute(GetClock).await
    }
    pub fn get_calendar(&self, date: impl RangeBounds<Date>) -> GetCalendarBuilder {
        use std::collections::Bound;

        GetCalendarBuilder(
            self,
            GetCalendar {
                start: if let Bound::Included(start) = date.start_bound() {
                    Some(*start)
                } else {
                    None
                },
                end: if let Bound::Included(end) = date.end_bound() {
                    Some(*end)
                } else {
                    None
                },
                date_type: DateType::default(),
            },
        )
    }
}

endpoint! {
    impl GET "/v2/account" = GetAccount => Account
    | trading
    | broker
    | account (|_, account_id| format!("/v2/accounts/{}", account_id));
    impl GET "/v2/clock" = GetClock => Clock
    | trading
    | broker;
    impl GET "/v2/calendar" = GetCalendar => Calendar { |this, request| request.json(this) };
}
