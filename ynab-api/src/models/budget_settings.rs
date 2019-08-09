/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetSettings {
    #[serde(rename = "date_format")]
    pub date_format: ::models::DateFormat,
    #[serde(rename = "currency_format")]
    pub currency_format: ::models::CurrencyFormat,
}

impl BudgetSettings {
    pub fn new(date_format: ::models::DateFormat, currency_format: ::models::CurrencyFormat) -> BudgetSettings {
        BudgetSettings {
            date_format: date_format,
            currency_format: currency_format,
        }
    }
}


