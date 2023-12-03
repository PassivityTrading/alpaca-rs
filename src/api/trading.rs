use super::*;

mod orders;
mod positions;
mod assets;

pub use orders::*;
pub use positions::*;
pub use assets::*;

/// Get account details.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GetAccount;

impl TradingClient {
    /// Gets the account data for this trading account.
    pub async fn get_account(&self) -> Result<Account> {
        self.execute(GetAccount).await
    }
}

endpoint! {
    impl GET "/account" = GetAccount => Account
    | trading
    | broker
    | account (|_, account_id| format!("/accounts/{}", account_id));
}
