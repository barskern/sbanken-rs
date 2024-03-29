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
pub struct TransactionDetail {
    #[serde(rename = "formattedAccountNumber", skip_serializing_if = "Option::is_none")]
    pub formatted_account_number: Option<String>,
    #[serde(rename = "transactionId")]
    pub transaction_id: i64,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "amountDescription", skip_serializing_if = "Option::is_none")]
    pub amount_description: Option<String>,
    #[serde(rename = "receiverName", skip_serializing_if = "Option::is_none")]
    pub receiver_name: Option<String>,
    #[serde(rename = "numericReference")]
    pub numeric_reference: i64,
    #[serde(rename = "payerName", skip_serializing_if = "Option::is_none")]
    pub payer_name: Option<String>,
    #[serde(rename = "registrationDate", skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<String>,
}

impl TransactionDetail {
    pub fn new(transaction_id: i64, numeric_reference: i64) -> TransactionDetail {
        TransactionDetail {
            formatted_account_number: None,
            transaction_id,
            cid: None,
            amount_description: None,
            receiver_name: None,
            numeric_reference,
            payer_name: None,
            registration_date: None,
        }
    }
}


