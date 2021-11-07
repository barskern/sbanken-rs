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
pub struct ListResultOfEFaktura {
    #[serde(rename = "availableItems")]
    pub available_items: i32,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::EFaktura>>,
}

impl ListResultOfEFaktura {
    pub fn new(available_items: i32) -> ListResultOfEFaktura {
        ListResultOfEFaktura {
            available_items,
            items: None,
        }
    }
}


