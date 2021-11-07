/*
 * Bff.ApiBeta : V2 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReservationType : 

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReservationType {
    #[serde(rename = "NotReservation")]
    NotReservation,
    #[serde(rename = "VisaReservation")]
    VisaReservation,
    #[serde(rename = "PurchaseReservation")]
    PurchaseReservation,
    #[serde(rename = "AtmReservation")]
    AtmReservation,

}

impl ToString for ReservationType {
    fn to_string(&self) -> String {
        match self {
            Self::NotReservation => String::from("NotReservation"),
            Self::VisaReservation => String::from("VisaReservation"),
            Self::PurchaseReservation => String::from("PurchaseReservation"),
            Self::AtmReservation => String::from("AtmReservation"),
        }
    }
}



