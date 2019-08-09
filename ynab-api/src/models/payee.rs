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
pub struct Payee {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    /// If a transfer payee, the account_id to which this payee transfers to
    #[serde(rename = "transfer_account_id")]
    pub transfer_account_id: String,
    /// Whether or not the payee has been deleted.  Deleted payees will only be included in delta requests.
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl Payee {
    pub fn new(id: String, name: String, transfer_account_id: String, deleted: bool) -> Payee {
        Payee {
            id: id,
            name: name,
            transfer_account_id: transfer_account_id,
            deleted: deleted,
        }
    }
}


