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
pub struct ScheduledTransactionSummary {
    #[serde(rename = "id")]
    pub id: String,
    /// The first date for which the Scheduled Transaction was scheduled.
    #[serde(rename = "date_first")]
    pub date_first: String,
    /// The next date for which the Scheduled Transaction is scheduled.
    #[serde(rename = "date_next")]
    pub date_next: String,
    #[serde(rename = "frequency")]
    pub frequency: String,
    /// The scheduled transaction amount in milliunits format
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "memo", skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    /// The scheduled transaction flag
    #[serde(rename = "flag_color")]
    pub flag_color: String,
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "payee_id", skip_serializing_if = "Option::is_none")]
    pub payee_id: Option<String>,
    #[serde(rename = "category_id")]
    pub category_id: String,
    /// If a transfer, the account_id which the scheduled transaction transfers to
    #[serde(rename = "transfer_account_id", skip_serializing_if = "Option::is_none")]
    pub transfer_account_id: Option<String>,
    /// Whether or not the scheduled transaction has been deleted.  Deleted scheduled transactions will only be included in delta requests.
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl ScheduledTransactionSummary {
    pub fn new(id: String, date_first: String, date_next: String, frequency: String, amount: i64, flag_color: String, account_id: String, category_id: String, deleted: bool) -> ScheduledTransactionSummary {
        ScheduledTransactionSummary {
            id: id,
            date_first: date_first,
            date_next: date_next,
            frequency: frequency,
            amount: amount,
            memo: None,
            flag_color: flag_color,
            account_id: account_id,
            payee_id: None,
            category_id: category_id,
            transfer_account_id: None,
            deleted: deleted,
        }
    }
}

/// 
#[derive(Debug, Serialize, Deserialize)]
pub enum Frequency {
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "everyOtherWeek")]
    EveryOtherWeek,
    #[serde(rename = "twiceAMonth")]
    TwiceAMonth,
    #[serde(rename = "every4Weeks")]
    Every4Weeks,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "everyOtherMonth")]
    EveryOtherMonth,
    #[serde(rename = "every3Months")]
    Every3Months,
    #[serde(rename = "every4Months")]
    Every4Months,
    #[serde(rename = "twiceAYear")]
    TwiceAYear,
    #[serde(rename = "yearly")]
    Yearly,
    #[serde(rename = "everyOtherYear")]
    EveryOtherYear,
}
/// The scheduled transaction flag
#[derive(Debug, Serialize, Deserialize)]
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

