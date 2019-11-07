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
pub struct HybridTransactionAllOf {
    /// Whether the hybrid transaction represents a regular transaction or a subtransaction
    #[serde(rename = "type")]
    pub _type: Type,
    /// For subtransaction types, this is the id of the parent transaction.  For transaction types, this id will be always be null.
    #[serde(rename = "parent_transaction_id", skip_serializing_if = "Option::is_none")]
    pub parent_transaction_id: Option<String>,
    #[serde(rename = "account_name")]
    pub account_name: String,
    #[serde(rename = "payee_name", skip_serializing_if = "Option::is_none")]
    pub payee_name: Option<String>,
    #[serde(rename = "category_name", skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
}

impl HybridTransactionAllOf {
    pub fn new(_type: Type, account_name: String) -> HybridTransactionAllOf {
        HybridTransactionAllOf {
            _type,
            parent_transaction_id: None,
            account_name,
            payee_name: None,
            category_name: None,
        }
    }
}

/// Whether the hybrid transaction represents a regular transaction or a subtransaction
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "transaction")]
    Transaction,
    #[serde(rename = "subtransaction")]
    Subtransaction,
}

