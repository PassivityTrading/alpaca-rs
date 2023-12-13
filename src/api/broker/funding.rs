use super::*;

#[with_builder(create_bank_relationship)]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Post(json) "/recipient_banks" in BrokerClient -> BankRelationship)]
pub struct CreateBankRelationship {
    #[required]
    pub name: String,
    #[required]
    pub bank_code: BankCode,
    #[required]
    pub bank_code_type: BankCodeType,
    #[required]
    pub account_number: String,
    pub country: Option<String>,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub street_address: Option<String>,
}

#[with_builder(create_ach_relationship)]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Post(json) "/ach_relationships" in BrokerClient -> AchRelationship)]
pub struct CreateAchRelationship {
    #[required]
    pub account_owner_name: String,
    #[required]
    pub bank_account_type: String,
    #[required]
    pub bank_account_number: String,
    #[required]
    pub bank_routing_number: String,
    pub nickname: Option<String>,
    pub processor_token: Option<String>,
    pub instant: Option<bool>,
}

#[with_builder(create_transfer)]
#[skip_serializing_none]
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Post(json) "/transfer" in BrokerClient -> Transfer)]
pub struct CreateTransfer {
    #[required]
    pub transfer_type: TransferType,
    pub relationship_id: Option<String>,
    pub bank_id: Option<String>,
    #[required]
    #[serde_as(as = "DisplayFromStr")]
    pub amount: f64,
    #[required]
    pub direction: Direction,
    #[required]
    pub timing: Timing,
    pub additional_information: Option<String>,
    pub fee_payment_method: Option<String>,
}
