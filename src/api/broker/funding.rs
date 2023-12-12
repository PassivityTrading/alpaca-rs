use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Post(json) "/recipient_banks" in BrokerClient -> BankRelationship)]
pub struct CreateBankRelationship {
    pub name: String,
    pub bank_code: BankCode,
    pub bank_code_type: BankCodeType,
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

#[derive(Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Post(json) "/ach_relationships" in BrokerClient -> AchRelationship)]
pub struct CreateAchRelationship {
    pub account_owner_name: String,
    pub bank_account_type: String,
    pub bank_account_number: String,
    pub bank_routing_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant: Option<bool>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, ClientEndpoint)]
#[endpoint(Post(json) "/transfer" in BrokerClient -> Transfer)]
pub struct CreateTransfer {
    pub transfer_type: TransferType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_id: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: f64,
    pub direction: Direction,
    pub timing: Timing,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_payment_method: Option<String>,
}
