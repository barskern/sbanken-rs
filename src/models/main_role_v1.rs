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
pub enum MainRoleV1 {
    #[serde(rename = "Owner")]
    Owner,
    #[serde(rename = "Disposal")]
    Disposal,
    #[serde(rename = "CoOwner")]
    CoOwner,
    #[serde(rename = "CoDebtor")]
    CoDebtor,
    #[serde(rename = "Collateral")]
    Collateral,
    #[serde(rename = "Guardian")]
    Guardian,
    #[serde(rename = "Access")]
    Access,
    #[serde(rename = "Addressee")]
    Addressee,
    #[serde(rename = "Guarantator")]
    Guarantator,
    #[serde(rename = "Undefined")]
    Undefined,

}

impl std::string::ToString for MainRoleV1 {
    fn to_string(&self) -> String {
        match self {
        
            Self::Owner => "Owner".to_string(),
        
            Self::Disposal => "Disposal".to_string(),
        
            Self::CoOwner => "CoOwner".to_string(),
        
            Self::CoDebtor => "CoDebtor".to_string(),
        
            Self::Collateral => "Collateral".to_string(),
        
            Self::Guardian => "Guardian".to_string(),
        
            Self::Access => "Access".to_string(),
        
            Self::Addressee => "Addressee".to_string(),
        
            Self::Guarantator => "Guarantator".to_string(),
        
            Self::Undefined => "Undefined".to_string(),
        
        }
    }
}




