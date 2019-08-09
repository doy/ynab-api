use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

use super::models::*;

mod accounts_api;
pub use self::accounts_api::{ AccountsApi, AccountsApiClient };
mod budgets_api;
pub use self::budgets_api::{ BudgetsApi, BudgetsApiClient };
mod categories_api;
pub use self::categories_api::{ CategoriesApi, CategoriesApiClient };
mod deprecated_api;
pub use self::deprecated_api::{ DeprecatedApi, DeprecatedApiClient };
mod months_api;
pub use self::months_api::{ MonthsApi, MonthsApiClient };
mod payee_locations_api;
pub use self::payee_locations_api::{ PayeeLocationsApi, PayeeLocationsApiClient };
mod payees_api;
pub use self::payees_api::{ PayeesApi, PayeesApiClient };
mod scheduled_transactions_api;
pub use self::scheduled_transactions_api::{ ScheduledTransactionsApi, ScheduledTransactionsApiClient };
mod transactions_api;
pub use self::transactions_api::{ TransactionsApi, TransactionsApiClient };
mod user_api;
pub use self::user_api::{ UserApi, UserApiClient };

pub mod configuration;
pub mod client;
