/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use reqwest;

use super::{Error, configuration};

pub struct PayeeLocationsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl PayeeLocationsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> PayeeLocationsApiClient {
        PayeeLocationsApiClient {
            configuration,
        }
    }
}

pub trait PayeeLocationsApi {
    fn get_payee_location_by_id(&self, budget_id: &str, payee_location_id: &str) -> Result<crate::models::PayeeLocationResponse, Error>;
    fn get_payee_locations(&self, budget_id: &str) -> Result<crate::models::PayeeLocationsResponse, Error>;
    fn get_payee_locations_by_payee(&self, budget_id: &str, payee_id: &str) -> Result<crate::models::PayeeLocationsResponse, Error>;
}

impl PayeeLocationsApi for PayeeLocationsApiClient {
    fn get_payee_location_by_id(&self, budget_id: &str, payee_location_id: &str) -> Result<crate::models::PayeeLocationResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/budgets/{budget_id}/payee_locations/{payee_location_id}", configuration.base_path, budget_id=crate::apis::urlencode(budget_id), payee_location_id=crate::apis::urlencode(payee_location_id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_payee_locations(&self, budget_id: &str) -> Result<crate::models::PayeeLocationsResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/budgets/{budget_id}/payee_locations", configuration.base_path, budget_id=crate::apis::urlencode(budget_id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_payee_locations_by_payee(&self, budget_id: &str, payee_id: &str) -> Result<crate::models::PayeeLocationsResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/budgets/{budget_id}/payees/{payee_id}/payee_locations", configuration.base_path, budget_id=crate::apis::urlencode(budget_id), payee_id=crate::apis::urlencode(payee_id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
