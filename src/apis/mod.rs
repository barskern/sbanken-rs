use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reqwest(_) => write!(f, "reqwest error"),
            Self::Serde(_) => write!(f, "serde error"),
            Self::Io(_) => write!(f, "i/o error"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Reqwest(e) => Some(e),
            Self::Serde(e) => Some(e),
            Self::Io(e) => Some(e),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

mod accounts_api;
pub use self::accounts_api::{ AccountsApi, AccountsApiClient };
mod cards_api;
pub use self::cards_api::{ CardsApi, CardsApiClient };
mod customers_api;
pub use self::customers_api::{ CustomersApi, CustomersApiClient };
mod e_fakturas_api;
pub use self::e_fakturas_api::{ EFakturasApi, EFakturasApiClient };
mod my_profiles_api;
pub use self::my_profiles_api::{ MyProfilesApi, MyProfilesApiClient };
mod payments_api;
pub use self::payments_api::{ PaymentsApi, PaymentsApiClient };
mod standing_orders_api;
pub use self::standing_orders_api::{ StandingOrdersApi, StandingOrdersApiClient };
mod transactions_api;
pub use self::transactions_api::{ TransactionsApi, TransactionsApiClient };
mod transfers_api;
pub use self::transfers_api::{ TransfersApi, TransfersApiClient };

pub mod configuration;
pub mod client;
