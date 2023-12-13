use super::*;

#[with_builder(create_bank_relationship)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
}

#[with_builder(create_ach_relationship)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant: Option<bool>,
}

#[with_builder(create_transfer)]
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Post(json) "/transfer" in BrokerClient -> Transfer)]
pub struct CreateTransfer {
    #[required]
    pub transfer_type: TransferType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_id: Option<String>,
    #[required]
    #[serde_as(as = "DisplayFromStr")]
    pub amount: f64,
    #[required]
    pub direction: Direction,
    #[required]
    pub timing: Timing,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_payment_method: Option<String>,
}
