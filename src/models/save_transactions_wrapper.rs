/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SaveTransactionsWrapper {
    #[serde(rename = "transaction", skip_serializing_if = "Option::is_none")]
    pub transaction: Option<crate::models::SaveTransaction>,
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<crate::models::SaveTransaction>>,
}

impl SaveTransactionsWrapper {
    pub fn new() -> SaveTransactionsWrapper {
        SaveTransactionsWrapper {
            transaction: None,
            transactions: None,
        }
    }
}


