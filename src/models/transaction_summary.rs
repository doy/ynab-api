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
pub struct TransactionSummary {
    #[serde(rename = "id")]
    pub id: String,
    /// The transaction date in ISO format (e.g. 2016-12-01)
    #[serde(rename = "date")]
    pub date: String,
    /// The transaction amount in milliunits format
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "memo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub memo: Option<Option<String>>,
    #[serde(rename = "cleared")]
    pub cleared: models::TransactionClearedStatus,
    /// Whether or not the transaction is approved
    #[serde(rename = "approved")]
    pub approved: bool,
    #[serde(rename = "flag_color", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub flag_color: Option<Option<models::TransactionFlagColor>>,
    /// The customized name of a transaction flag
    #[serde(rename = "flag_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub flag_name: Option<Option<String>>,
    #[serde(rename = "account_id")]
    pub account_id: uuid::Uuid,
    #[serde(rename = "payee_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payee_id: Option<Option<uuid::Uuid>>,
    #[serde(rename = "category_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<Option<uuid::Uuid>>,
    /// If a transfer transaction, the account to which it transfers
    #[serde(rename = "transfer_account_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub transfer_account_id: Option<Option<uuid::Uuid>>,
    /// If a transfer transaction, the id of transaction on the other side of the transfer
    #[serde(rename = "transfer_transaction_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub transfer_transaction_id: Option<Option<String>>,
    /// If transaction is matched, the id of the matched transaction
    #[serde(rename = "matched_transaction_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub matched_transaction_id: Option<Option<String>>,
    /// If the transaction was imported, this field is a unique (by account) import identifier.  If this transaction was imported through File Based Import or Direct Import and not through the API, the import_id will have the format: 'YNAB:[milliunit_amount]:[iso_date]:[occurrence]'.  For example, a transaction dated 2015-12-30 in the amount of -$294.23 USD would have an import_id of 'YNAB:-294230:2015-12-30:1'.  If a second transaction on the same account was imported and had the same date and same amount, its import_id would be 'YNAB:-294230:2015-12-30:2'.
    #[serde(rename = "import_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub import_id: Option<Option<String>>,
    /// If the transaction was imported, the payee name that was used when importing and before applying any payee rename rules
    #[serde(rename = "import_payee_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub import_payee_name: Option<Option<String>>,
    /// If the transaction was imported, the original payee name as it appeared on the statement
    #[serde(rename = "import_payee_name_original", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub import_payee_name_original: Option<Option<String>>,
    /// If the transaction is a debt/loan account transaction, the type of transaction
    #[serde(rename = "debt_transaction_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub debt_transaction_type: Option<Option<DebtTransactionType>>,
    /// Whether or not the transaction has been deleted.  Deleted transactions will only be included in delta requests.
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl TransactionSummary {
    pub fn new(id: String, date: String, amount: i64, cleared: models::TransactionClearedStatus, approved: bool, account_id: uuid::Uuid, deleted: bool) -> TransactionSummary {
        TransactionSummary {
            id,
            date,
            amount,
            memo: None,
            cleared,
            approved,
            flag_color: None,
            flag_name: None,
            account_id,
            payee_id: None,
            category_id: None,
            transfer_account_id: None,
            transfer_transaction_id: None,
            matched_transaction_id: None,
            import_id: None,
            import_payee_name: None,
            import_payee_name_original: None,
            debt_transaction_type: None,
            deleted,
        }
    }
}
/// If the transaction is a debt/loan account transaction, the type of transaction
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DebtTransactionType {
    #[serde(rename = "payment")]
    Payment,
    #[serde(rename = "refund")]
    Refund,
    #[serde(rename = "fee")]
    Fee,
    #[serde(rename = "interest")]
    Interest,
    #[serde(rename = "escrow")]
    Escrow,
    #[serde(rename = "balanceAdjustment")]
    BalanceAdjustment,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "charge")]
    Charge,
}

impl Default for DebtTransactionType {
    fn default() -> DebtTransactionType {
        Self::Payment
    }
}

