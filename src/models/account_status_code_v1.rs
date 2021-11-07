/*
 * SBanken API
 *
 * API for accessing bank information in SBanken
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountStatusCodeV1 {
    #[serde(rename = "ActiveAccount")]
    ActiveAccount,
    #[serde(rename = "Blocked")]
    Blocked,
    #[serde(rename = "BlockedDeposit")]
    BlockedDeposit,
    #[serde(rename = "BlockedWithdrawel")]
    BlockedWithdrawel,
    #[serde(rename = "Closed")]
    Closed,
    #[serde(rename = "Undefined")]
    Undefined,

}

impl ToString for AccountStatusCodeV1 {
    fn to_string(&self) -> String {
        match self {
            Self::ActiveAccount => String::from("ActiveAccount"),
            Self::Blocked => String::from("Blocked"),
            Self::BlockedDeposit => String::from("BlockedDeposit"),
            Self::BlockedWithdrawel => String::from("BlockedWithdrawel"),
            Self::Closed => String::from("Closed"),
            Self::Undefined => String::from("Undefined"),
        }
    }
}




