/*
 * Bff.ApiBeta : V2 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArchiveTransaction {
    #[serde(rename = "transactionId", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "accountingDate")]
    pub accounting_date: String,
    #[serde(rename = "interestDate", skip_serializing_if = "Option::is_none")]
    pub interest_date: Option<String>,
    #[serde(rename = "amount")]
    pub amount: f32,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "transactionType", skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
    #[serde(rename = "transactionTypeCode")]
    pub transaction_type_code: i32,
    #[serde(rename = "transactionTypeText", skip_serializing_if = "Option::is_none")]
    pub transaction_type_text: Option<String>,
    #[serde(rename = "source")]
    pub source: crate::models::SourceType,
    #[serde(rename = "cardDetails", skip_serializing_if = "Option::is_none")]
    pub card_details: Option<Box<crate::models::CardDetails>>,
    #[serde(rename = "cardDetailsSpecified")]
    pub card_details_specified: bool,
}

impl ArchiveTransaction {
    pub fn new(accounting_date: String, amount: f32, transaction_type_code: i32, source: crate::models::SourceType, card_details_specified: bool) -> ArchiveTransaction {
        ArchiveTransaction {
            transaction_id: None,
            accounting_date,
            interest_date: None,
            amount,
            text: None,
            transaction_type: None,
            transaction_type_code,
            transaction_type_text: None,
            source,
            card_details: None,
            card_details_specified,
        }
    }
}


