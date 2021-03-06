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
pub struct EFakturaSimpleV1 {
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
    #[serde(rename = "originalDueDate", skip_serializing_if = "Option::is_none")]
    pub original_due_date: Option<String>,
    #[serde(rename = "originalAmount", skip_serializing_if = "Option::is_none")]
    pub original_amount: Option<f64>,
    #[serde(rename = "minimumAmount", skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<f64>,
    #[serde(rename = "notificationDate", skip_serializing_if = "Option::is_none")]
    pub notification_date: Option<String>,
    #[serde(rename = "issuerName", skip_serializing_if = "Option::is_none")]
    pub issuer_name: Option<String>,
}

impl EFakturaSimpleV1 {
    pub fn new() -> EFakturaSimpleV1 {
        EFakturaSimpleV1 {
            e_faktura_id: None,
            issuer_id: None,
            e_faktura_reference: None,
            document_type: None,
            status: None,
            kid: None,
            original_due_date: None,
            original_amount: None,
            minimum_amount: None,
            notification_date: None,
            issuer_name: None,
        }
    }
}


