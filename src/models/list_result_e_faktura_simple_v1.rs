/*
 * SBanken API
 *
 * API for accessing bank information in SBanken
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListResultEFakturaSimpleV1 {
    #[serde(rename = "availableItems", skip_serializing_if = "Option::is_none")]
    pub available_items: Option<i32>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::EFakturaSimpleV1>>,
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

impl ListResultEFakturaSimpleV1 {
    pub fn new() -> ListResultEFakturaSimpleV1 {
        ListResultEFakturaSimpleV1 {
            available_items: None,
            items: None,
            error_type: None,
            is_error: None,
            error_code: None,
            error_message: None,
            trace_id: None,
        }
    }
}


