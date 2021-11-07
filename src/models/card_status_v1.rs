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
pub enum CardStatusV1 {
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Inactive")]
    Inactive,
    #[serde(rename = "Renewal")]
    Renewal,
    #[serde(rename = "Deleted")]
    Deleted,
    #[serde(rename = "Blocked")]
    Blocked,

}

impl ToString for CardStatusV1 {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("Unknown"),
            Self::Active => String::from("Active"),
            Self::Inactive => String::from("Inactive"),
            Self::Renewal => String::from("Renewal"),
            Self::Deleted => String::from("Deleted"),
            Self::Blocked => String::from("Blocked"),
        }
    }
}




