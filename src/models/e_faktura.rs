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
pub struct EFaktura {
    #[serde(rename = "eFakturaId", skip_serializing_if = "Option::is_none")]
    pub e_faktura_id: Option<String>,
    #[serde(rename = "issuerId", skip_serializing_if = "Option::is_none")]
    pub issuer_id: Option<String>,
    #[serde(rename = "eFakturaReference", skip_serializing_if = "Option::is_none")]
    pub e_faktura_reference: Option<String>,
    #[serde(rename = "documentType", skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "kid", skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(rename = "originalDueDate")]
    pub original_due_date: String,
    #[serde(rename = "originalAmount")]
    pub original_amount: f32,
    #[serde(rename = "minimumAmount")]
    pub minimum_amount: f32,
    #[serde(rename = "updatedDueDate", skip_serializing_if = "Option::is_none")]
    pub updated_due_date: Option<String>,
    #[serde(rename = "updatedAmount", skip_serializing_if = "Option::is_none")]
    pub updated_amount: Option<f32>,
    #[serde(rename = "notificationDate")]
    pub notification_date: String,
    #[serde(rename = "creditAccountNumber", skip_serializing_if = "Option::is_none")]
    pub credit_account_number: Option<String>,
    #[serde(rename = "issuerName", skip_serializing_if = "Option::is_none")]
    pub issuer_name: Option<String>,
}

impl EFaktura {
    pub fn new(original_due_date: String, original_amount: f32, minimum_amount: f32, notification_date: String) -> EFaktura {
        EFaktura {
            e_faktura_id: None,
            issuer_id: None,
            e_faktura_reference: None,
            document_type: None,
            status: None,
            kid: None,
            original_due_date,
            original_amount,
            minimum_amount,
            updated_due_date: None,
            updated_amount: None,
            notification_date,
            credit_account_number: None,
            issuer_name: None,
        }
    }
}


