/*
 * SBanken API
 *
 * API for accessing bank information in SBanken
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */


use serde::{Deserialize, Serialize};

/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StandingOrderFrequencyV1 {
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "Weekly")]
    Weekly,
    #[serde(rename = "Biweekly")]
    Biweekly,
    #[serde(rename = "Semimonthly")]
    Semimonthly,
    #[serde(rename = "Monthly")]
    Monthly,
    #[serde(rename = "Bimonthly")]
    Bimonthly,
    #[serde(rename = "Quarterly")]
    Quarterly,
    #[serde(rename = "Yearly")]
    Yearly,
    #[serde(rename = "Daily")]
    Daily,

}

impl std::string::ToString for StandingOrderFrequencyV1 {
    fn to_string(&self) -> String {
        match self {
        
            Self::Other => "Other".to_string(),
        
            Self::Weekly => "Weekly".to_string(),
        
            Self::Biweekly => "Biweekly".to_string(),
        
            Self::Semimonthly => "Semimonthly".to_string(),
        
            Self::Monthly => "Monthly".to_string(),
        
            Self::Bimonthly => "Bimonthly".to_string(),
        
            Self::Quarterly => "Quarterly".to_string(),
        
            Self::Yearly => "Yearly".to_string(),
        
            Self::Daily => "Daily".to_string(),
        
        }
    }
}





