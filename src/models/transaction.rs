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
pub struct Transaction {
    #[serde(rename = "accountingDate")]
    pub accounting_date: String,
    #[serde(rename = "interestDate", skip_serializing_if = "Option::is_none")]
    pub interest_date: Option<String>,
    #[serde(rename = "otherAccountNumber", skip_serializing_if = "Option::is_none")]
    pub other_account_number: Option<String>,
    #[serde(rename = "otherAccountNumberSpecified")]
    pub other_account_number_specified: bool,
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
    #[serde(rename = "isReservation")]
    pub is_reservation: bool,
    #[serde(rename = "reservationType", skip_serializing_if = "Option::is_none")]
    pub reservation_type: Option<crate::models::ReservationType>,
    #[serde(rename = "source")]
    pub source: crate::models::SourceType,
    #[serde(rename = "cardDetailsSpecified")]
    pub card_details_specified: bool,
    #[serde(rename = "cardDetails", skip_serializing_if = "Option::is_none")]
    pub card_details: Option<Box<crate::models::CardDetails>>,
    #[serde(rename = "transactionDetailSpecified")]
    pub transaction_detail_specified: bool,
    #[serde(rename = "transactionDetail", skip_serializing_if = "Option::is_none")]
    pub transaction_detail: Option<Box<crate::models::TransactionDetail>>,
}

impl Transaction {
    pub fn new(accounting_date: String, other_account_number_specified: bool, amount: f32, transaction_type_code: i32, is_reservation: bool, source: crate::models::SourceType, card_details_specified: bool, transaction_detail_specified: bool) -> Transaction {
        Transaction {
            accounting_date,
            interest_date: None,
            other_account_number: None,
            other_account_number_specified,
            amount,
            text: None,
            transaction_type: None,
            transaction_type_code,
            transaction_type_text: None,
            is_reservation,
            reservation_type: None,
            source,
            card_details_specified,
            card_details: None,
            transaction_detail_specified,
            transaction_detail: None,
        }
    }
}


