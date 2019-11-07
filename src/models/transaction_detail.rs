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
pub struct TransactionDetail {
    #[serde(rename = "id")]
    pub id: String,
    /// The transaction date in ISO format (e.g. 2016-12-01)
    #[serde(rename = "date")]
    pub date: String,
    /// The transaction amount in milliunits format
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "memo", skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    /// The cleared status of the transaction
    #[serde(rename = "cleared")]
    pub cleared: Cleared,
    /// Whether or not the transaction is approved
    #[serde(rename = "approved")]
    pub approved: bool,
    /// The transaction flag
    #[serde(rename = "flag_color", skip_serializing_if = "Option::is_none")]
    pub flag_color: Option<FlagColor>,
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "payee_id", skip_serializing_if = "Option::is_none")]
    pub payee_id: Option<String>,
    #[serde(rename = "category_id", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// If a transfer transaction, the account to which it transfers
    #[serde(rename = "transfer_account_id", skip_serializing_if = "Option::is_none")]
    pub transfer_account_id: Option<String>,
    /// If a transfer transaction, the id of transaction on the other side of the transfer
    #[serde(rename = "transfer_transaction_id", skip_serializing_if = "Option::is_none")]
    pub transfer_transaction_id: Option<String>,
    /// If transaction is matched, the id of the matched transaction
    #[serde(rename = "matched_transaction_id", skip_serializing_if = "Option::is_none")]
    pub matched_transaction_id: Option<String>,
    /// If the Transaction was imported, this field is a unique (by account) import identifier.  If this transaction was imported through File Based Import or Direct Import and not through the API, the import_id will have the format: 'YNAB:[milliunit_amount]:[iso_date]:[occurrence]'.  For example, a transaction dated 2015-12-30 in the amount of -$294.23 USD would have an import_id of 'YNAB:-294230:2015-12-30:1'.  If a second transaction on the same account was imported and had the same date and same amount, its import_id would be 'YNAB:-294230:2015-12-30:2'.
    #[serde(rename = "import_id", skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    /// Whether or not the transaction has been deleted.  Deleted transactions will only be included in delta requests.
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "account_name")]
    pub account_name: String,
    #[serde(rename = "payee_name", skip_serializing_if = "Option::is_none")]
    pub payee_name: Option<String>,
    #[serde(rename = "category_name", skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    /// If a split transaction, the subtransactions.
    #[serde(rename = "subtransactions")]
    pub subtransactions: Vec<crate::models::SubTransaction>,
}

impl TransactionDetail {
    pub fn new(id: String, date: String, amount: i64, cleared: Cleared, approved: bool, account_id: String, deleted: bool, account_name: String, subtransactions: Vec<crate::models::SubTransaction>) -> TransactionDetail {
        TransactionDetail {
            id,
            date,
            amount,
            memo: None,
            cleared,
            approved,
            flag_color: None,
            account_id,
            payee_id: None,
            category_id: None,
            transfer_account_id: None,
            transfer_transaction_id: None,
            matched_transaction_id: None,
            import_id: None,
            deleted,
            account_name,
            payee_name: None,
            category_name: None,
            subtransactions,
        }
    }
}

/// The cleared status of the transaction
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Cleared {
    #[serde(rename = "cleared")]
    Cleared,
    #[serde(rename = "uncleared")]
    Uncleared,
    #[serde(rename = "reconciled")]
    Reconciled,
}
/// The transaction flag
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FlagColor {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "purple")]
    Purple,
}

