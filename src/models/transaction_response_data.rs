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
pub struct TransactionResponseData {
    #[serde(rename = "transaction")]
    pub transaction: crate::models::TransactionDetail,
}

impl TransactionResponseData {
    pub fn new(transaction: crate::models::TransactionDetail) -> TransactionResponseData {
        TransactionResponseData {
            transaction,
        }
    }
}


