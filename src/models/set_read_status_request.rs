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
pub struct SetReadStatusRequest {
    #[serde(rename = "status")]
    pub status: crate::models::Status,
}

impl SetReadStatusRequest {
    pub fn new(status: crate::models::Status) -> SetReadStatusRequest {
        SetReadStatusRequest {
            status,
        }
    }
}

