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
pub struct PaymentV1 {
    #[serde(rename = "paymentId", skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(rename = "recipientAccountNumber", skip_serializing_if = "Option::is_none")]
    pub recipient_account_number: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "dueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "kid", skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "allowedNewStatusTypes", skip_serializing_if = "Option::is_none")]
    pub allowed_new_status_types: Option<Vec<crate::models::NewStatusTypeV1>>,
    #[serde(rename = "statusDetails", skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "productType", skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[serde(rename = "paymentType", skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    #[serde(rename = "paymentNumber", skip_serializing_if = "Option::is_none")]
    pub payment_number: Option<i32>,
    #[serde(rename = "beneficiaryName", skip_serializing_if = "Option::is_none")]
    pub beneficiary_name: Option<String>,
}

impl PaymentV1 {
    pub fn new() -> PaymentV1 {
        PaymentV1 {
            payment_id: None,
            recipient_account_number: None,
            amount: None,
            due_date: None,
            kid: None,
            text: None,
            is_active: None,
            status: None,
            allowed_new_status_types: None,
            status_details: None,
            product_type: None,
            payment_type: None,
            payment_number: None,
            beneficiary_name: None,
        }
    }
}


