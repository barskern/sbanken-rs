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
pub struct TransactionV1 {
    #[serde(rename = "accountingDate", skip_serializing_if = "Option::is_none")]
    pub accounting_date: Option<String>,
    #[serde(rename = "interestDate", skip_serializing_if = "Option::is_none")]
    pub interest_date: Option<String>,
    #[serde(rename = "otherAccountNumber", skip_serializing_if = "Option::is_none")]
    pub other_account_number: Option<String>,
    #[serde(rename = "otherAccountNumberSpecified", skip_serializing_if = "Option::is_none")]
    pub other_account_number_specified: Option<bool>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "transactionType", skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
    #[serde(rename = "transactionTypeCode", skip_serializing_if = "Option::is_none")]
    pub transaction_type_code: Option<i32>,
    #[serde(rename = "transactionTypeText", skip_serializing_if = "Option::is_none")]
    pub transaction_type_text: Option<String>,
    #[serde(rename = "isReservation", skip_serializing_if = "Option::is_none")]
    pub is_reservation: Option<bool>,
    #[serde(rename = "reservationType", skip_serializing_if = "Option::is_none")]
    pub reservation_type: Option<crate::models::ReservationTypeV1>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::models::SourceTypeV1>,
    #[serde(rename = "cardDetailsSpecified", skip_serializing_if = "Option::is_none")]
    pub card_details_specified: Option<bool>,
    #[serde(rename = "cardDetails", skip_serializing_if = "Option::is_none")]
    pub card_details: Option<Box<crate::models::CardDetailsV1>>,
    #[serde(rename = "transactionDetailSpecified", skip_serializing_if = "Option::is_none")]
    pub transaction_detail_specified: Option<bool>,
    #[serde(rename = "transactionDetail", skip_serializing_if = "Option::is_none")]
    pub transaction_detail: Option<Box<crate::models::TransactionDetailV1>>,
}

impl TransactionV1 {
    pub fn new() -> TransactionV1 {
        TransactionV1 {
            accounting_date: None,
            interest_date: None,
            other_account_number: None,
            other_account_number_specified: None,
            amount: None,
            text: None,
            transaction_type: None,
            transaction_type_code: None,
            transaction_type_text: None,
            is_reservation: None,
            reservation_type: None,
            source: None,
            card_details_specified: None,
            card_details: None,
            transaction_detail_specified: None,
            transaction_detail: None,
        }
    }
}


