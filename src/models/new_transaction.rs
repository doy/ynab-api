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
pub struct NewTransaction {
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<uuid::Uuid>,
    /// The transaction date in ISO format (e.g. 2016-12-01).  Future dates (scheduled transactions) are not permitted.  Split transaction dates cannot be changed and if a different date is supplied it will be ignored.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// The transaction amount in milliunits format.  Split transaction amounts cannot be changed and if a different amount is supplied it will be ignored.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The payee for the transaction.  To create a transfer between two accounts, use the account transfer payee pointing to the target account.  Account transfer payees are specified as `transfer_payee_id` on the account resource.
    #[serde(rename = "payee_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payee_id: Option<Option<uuid::Uuid>>,
    /// The payee name.  If a `payee_name` value is provided and `payee_id` has a null value, the `payee_name` value will be used to resolve the payee by either (1) a matching payee rename rule (only if `import_id` is also specified) or (2) a payee with the same name or (3) creation of a new payee.
    #[serde(rename = "payee_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payee_name: Option<Option<String>>,
    /// The category for the transaction.  To configure a split transaction, you can specify null for `category_id` and provide a `subtransactions` array as part of the transaction object.  If an existing transaction is a split, the `category_id` cannot be changed.  Credit Card Payment categories are not permitted and will be ignored if supplied.
    #[serde(rename = "category_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<Option<uuid::Uuid>>,
    #[serde(rename = "memo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub memo: Option<Option<String>>,
    #[serde(rename = "cleared", skip_serializing_if = "Option::is_none")]
    pub cleared: Option<models::TransactionClearedStatus>,
    /// Whether or not the transaction is approved.  If not supplied, transaction will be unapproved by default.
    #[serde(rename = "approved", skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
    #[serde(rename = "flag_color", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub flag_color: Option<Option<models::TransactionFlagColor>>,
    /// An array of subtransactions to configure a transaction as a split. Updating `subtransactions` on an existing split transaction is not supported.
    #[serde(rename = "subtransactions", skip_serializing_if = "Option::is_none")]
    pub subtransactions: Option<Vec<models::SaveSubTransaction>>,
    /// If specified, a new transaction will be assigned this `import_id` and considered \"imported\".  We will also attempt to match this imported transaction to an existing \"user-entered\" transaction on the same account, with the same amount, and with a date +/-10 days from the imported transaction date.<br><br>Transactions imported through File Based Import or Direct Import (not through the API) are assigned an import_id in the format: 'YNAB:[milliunit_amount]:[iso_date]:[occurrence]'. For example, a transaction dated 2015-12-30 in the amount of -$294.23 USD would have an import_id of 'YNAB:-294230:2015-12-30:1'.  If a second transaction on the same account was imported and had the same date and same amount, its import_id would be 'YNAB:-294230:2015-12-30:2'.  Using a consistent format will prevent duplicates through Direct Import and File Based Import.<br><br>If import_id is omitted or specified as null, the transaction will be treated as a \"user-entered\" transaction. As such, it will be eligible to be matched against transactions later being imported (via DI, FBI, or API).
    #[serde(rename = "import_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub import_id: Option<Option<String>>,
}

impl NewTransaction {
    pub fn new() -> NewTransaction {
        NewTransaction {
            account_id: None,
            date: None,
            amount: None,
            payee_id: None,
            payee_name: None,
            category_id: None,
            memo: None,
            cleared: None,
            approved: None,
            flag_color: None,
            subtransactions: None,
            import_id: None,
        }
    }
}

