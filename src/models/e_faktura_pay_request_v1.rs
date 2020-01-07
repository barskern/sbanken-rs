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



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EFakturaPayRequestV1 {
    #[serde(rename = "eFakturaId", skip_serializing_if = "Option::is_none")]
    pub e_faktura_id: Option<String>,
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "payOnlyMinimumAmount", skip_serializing_if = "Option::is_none")]
    pub pay_only_minimum_amount: Option<bool>,
}

impl EFakturaPayRequestV1 {
    pub fn new() -> EFakturaPayRequestV1 {
        EFakturaPayRequestV1 {
            e_faktura_id: None,
            account_id: None,
            pay_only_minimum_amount: None,
        }
    }
}


