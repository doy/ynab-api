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
pub struct SaveTransactionsResponseData {
    /// The transaction ids that were saved
    #[serde(rename = "transaction_ids")]
    pub transaction_ids: Vec<String>,
    #[serde(rename = "transaction", skip_serializing_if = "Option::is_none")]
    pub transaction: Option<Box<models::TransactionDetail>>,
    /// If multiple transactions were specified, the transactions that were saved
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<models::TransactionDetail>>,
    /// If multiple transactions were specified, a list of import_ids that were not created because of an existing `import_id` found on the same account
    #[serde(rename = "duplicate_import_ids", skip_serializing_if = "Option::is_none")]
    pub duplicate_import_ids: Option<Vec<String>>,
    /// The knowledge of the server
    #[serde(rename = "server_knowledge")]
    pub server_knowledge: i64,
}

impl SaveTransactionsResponseData {
    pub fn new(transaction_ids: Vec<String>, server_knowledge: i64) -> SaveTransactionsResponseData {
        SaveTransactionsResponseData {
            transaction_ids,
            transaction: None,
            transactions: None,
            duplicate_import_ids: None,
            server_knowledge,
        }
    }
}

