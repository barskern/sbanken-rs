/*
 * SBanken API
 *
 * API for accessing bank information in SBanken
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

use async_trait::async_trait;
use std::sync::Arc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use reqwest;

use super::{Error, configuration};

pub struct TransactionsApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl TransactionsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> TransactionsApiClient {
        TransactionsApiClient {
            configuration,
        }
    }
}

#[async_trait]
pub trait TransactionsApi {
    async fn get_transactions(&self, account_id: &str, customer_id: Option<&str>, start_date: Option<String>, end_date: Option<String>, index: Option<i32>, length: Option<i32>) -> Result<crate::models::ListResultTransactionV1, Error>;
}

#[async_trait]
impl TransactionsApi for TransactionsApiClient {
    async fn get_transactions(&self, account_id: &str, customer_id: Option<&str>, start_date: Option<String>, end_date: Option<String>, index: Option<i32>, length: Option<i32>) -> Result<crate::models::ListResultTransactionV1, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/exec.bank/api/v1/Transactions/{accountId}", configuration.base_path, accountId=crate::apis::urlencode(account_id));
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = start_date {
            req_builder = req_builder.query(&[("startDate", &s.to_string())]);
        }
        if let Some(ref s) = end_date {
            req_builder = req_builder.query(&[("endDate", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("index", &s.to_string())]);
        }
        if let Some(ref s) = length {
            req_builder = req_builder.query(&[("length", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(param_value) = customer_id {
            req_builder = req_builder.header("customerId", param_value.to_string());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

}