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
pub struct ItemResultCustomerV1 {
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<crate::models::CustomerV1>,
    #[serde(rename = "errorType", skip_serializing_if = "Option::is_none")]
    pub error_type: Option<crate::models::ErrorType>,
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
    #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "traceId", skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

impl ItemResultCustomerV1 {
    pub fn new() -> ItemResultCustomerV1 {
        ItemResultCustomerV1 {
            item: None,
            error_type: None,
            is_error: None,
            error_code: None,
            error_message: None,
            trace_id: None,
        }
    }
}


