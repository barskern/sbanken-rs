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
pub struct Customer {
    #[serde(rename = "customerId", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "postalAddress", skip_serializing_if = "Option::is_none")]
    pub postal_address: Option<Box<crate::models::Address>>,
    #[serde(rename = "streetAddress", skip_serializing_if = "Option::is_none")]
    pub street_address: Option<Box<crate::models::Address>>,
    #[serde(rename = "phoneNumbers", skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<crate::models::PhoneNumber>>,
}

impl Customer {
    pub fn new() -> Customer {
        Customer {
            customer_id: None,
            first_name: None,
            last_name: None,
            email_address: None,
            date_of_birth: None,
            postal_address: None,
            street_address: None,
            phone_numbers: None,
        }
    }
}

