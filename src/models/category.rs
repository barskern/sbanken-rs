/*
 * Bff.ApiBeta : V2 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Category : 

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "General")]
    General,
    #[serde(rename = "News")]
    News,

}

impl ToString for Category {
    fn to_string(&self) -> String {
        match self {
            Self::General => String::from("General"),
            Self::News => String::from("News"),
        }
    }
}




