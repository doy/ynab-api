/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.ynab.com
 *
 * The version of the OpenAPI document: 1.72.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetSettingsResponseData {
    #[serde(rename = "settings")]
    pub settings: Box<models::BudgetSettings>,
}

impl BudgetSettingsResponseData {
    pub fn new(settings: models::BudgetSettings) -> BudgetSettingsResponseData {
        BudgetSettingsResponseData {
            settings: Box::new(settings),
        }
    }
}

