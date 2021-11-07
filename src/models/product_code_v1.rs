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
pub enum ProductCodeV1 {
    #[serde(rename = "DebitCard")]
    DebitCard,
    #[serde(rename = "DebitCardCL")]
    DebitCardCL,
    #[serde(rename = "CreditCard")]
    CreditCard,
    #[serde(rename = "CreditCardCL")]
    CreditCardCL,
    #[serde(rename = "DebitCardYouth")]
    DebitCardYouth,
    #[serde(rename = "DebitCardYouthCL")]
    DebitCardYouthCL,
    #[serde(rename = "X2XCard")]
    X2XCard,
    #[serde(rename = "X2XCardChild")]
    X2XCardChild,
    #[serde(rename = "X2XCardChildNet")]
    X2XCardChildNet,
    #[serde(rename = "ElectronCard")]
    ElectronCard,
    #[serde(rename = "Unknown")]
    Unknown,

}

impl ToString for ProductCodeV1 {
    fn to_string(&self) -> String {
        match self {
            Self::DebitCard => String::from("DebitCard"),
            Self::DebitCardCL => String::from("DebitCardCL"),
            Self::CreditCard => String::from("CreditCard"),
            Self::CreditCardCL => String::from("CreditCardCL"),
            Self::DebitCardYouth => String::from("DebitCardYouth"),
            Self::DebitCardYouthCL => String::from("DebitCardYouthCL"),
            Self::X2XCard => String::from("X2XCard"),
            Self::X2XCardChild => String::from("X2XCardChild"),
            Self::X2XCardChildNet => String::from("X2XCardChildNet"),
            Self::ElectronCard => String::from("ElectronCard"),
            Self::Unknown => String::from("Unknown"),
        }
    }
}




