use std::sync::Arc;

use super::configuration::Configuration;

pub struct APIClient {
    accounts_api: Box<dyn crate::apis::AccountsApi>,
    cards_api: Box<dyn crate::apis::CardsApi>,
    customers_api: Box<dyn crate::apis::CustomersApi>,
    e_fakturas_api: Box<dyn crate::apis::EFakturasApi>,
    my_profiles_api: Box<dyn crate::apis::MyProfilesApi>,
    payments_api: Box<dyn crate::apis::PaymentsApi>,
    standing_orders_api: Box<dyn crate::apis::StandingOrdersApi>,
    transactions_api: Box<dyn crate::apis::TransactionsApi>,
    transfers_api: Box<dyn crate::apis::TransfersApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let arc = Arc::new(configuration);

        APIClient {
            accounts_api: Box::new(crate::apis::AccountsApiClient::new(arc.clone())),
            cards_api: Box::new(crate::apis::CardsApiClient::new(arc.clone())),
            customers_api: Box::new(crate::apis::CustomersApiClient::new(arc.clone())),
            e_fakturas_api: Box::new(crate::apis::EFakturasApiClient::new(arc.clone())),
            my_profiles_api: Box::new(crate::apis::MyProfilesApiClient::new(arc.clone())),
            payments_api: Box::new(crate::apis::PaymentsApiClient::new(arc.clone())),
            standing_orders_api: Box::new(crate::apis::StandingOrdersApiClient::new(arc.clone())),
            transactions_api: Box::new(crate::apis::TransactionsApiClient::new(arc.clone())),
            transfers_api: Box::new(crate::apis::TransfersApiClient::new(arc.clone())),
        }
    }

    pub fn accounts_api(&self) -> &dyn crate::apis::AccountsApi{
        self.accounts_api.as_ref()
    }

    pub fn cards_api(&self) -> &dyn crate::apis::CardsApi{
        self.cards_api.as_ref()
    }

    pub fn customers_api(&self) -> &dyn crate::apis::CustomersApi{
        self.customers_api.as_ref()
    }

    pub fn e_fakturas_api(&self) -> &dyn crate::apis::EFakturasApi{
        self.e_fakturas_api.as_ref()
    }

    pub fn my_profiles_api(&self) -> &dyn crate::apis::MyProfilesApi{
        self.my_profiles_api.as_ref()
    }

    pub fn payments_api(&self) -> &dyn crate::apis::PaymentsApi{
        self.payments_api.as_ref()
    }

    pub fn standing_orders_api(&self) -> &dyn crate::apis::StandingOrdersApi{
        self.standing_orders_api.as_ref()
    }

    pub fn transactions_api(&self) -> &dyn crate::apis::TransactionsApi{
        self.transactions_api.as_ref()
    }

    pub fn transfers_api(&self) -> &dyn crate::apis::TransfersApi{
        self.transfers_api.as_ref()
    }

}
