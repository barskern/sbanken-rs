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
pub enum NewStatusTypeV1 {
    #[serde(rename = "Stopped")]
    Stopped,
    #[serde(rename = "Reactivated")]
    Reactivated,
    #[serde(rename = "IgnoreLimit")]
    IgnoreLimit,

}

impl std::string::ToString for NewStatusTypeV1 {
    fn to_string(&self) -> String {
        match self {
        
            Self::Stopped => "Stopped".to_string(),
        
            Self::Reactivated => "Reactivated".to_string(),
        
            Self::IgnoreLimit => "IgnoreLimit".to_string(),
        
        }
    }
}





