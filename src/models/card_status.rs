/*
 * Bff.ApiBeta : V2 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CardStatus : 

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CardStatus {
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

impl ToString for CardStatus {
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




