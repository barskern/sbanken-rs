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
pub struct AccountProductV1 {
    #[serde(rename = "productNumber", skip_serializing_if = "Option::is_none")]
    pub product_number: Option<String>,
    #[serde(rename = "accountProductCode", skip_serializing_if = "Option::is_none")]
    pub account_product_code: Option<crate::models::AccountProductCodeV1>,
}

impl AccountProductV1 {
    pub fn new() -> AccountProductV1 {
        AccountProductV1 {
            product_number: None,
            account_product_code: None,
        }
    }
}


