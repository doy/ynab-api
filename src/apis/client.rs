use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    accounts_api: Box<crate::apis::AccountsApi>,
    budgets_api: Box<crate::apis::BudgetsApi>,
    categories_api: Box<crate::apis::CategoriesApi>,
    deprecated_api: Box<crate::apis::DeprecatedApi>,
    months_api: Box<crate::apis::MonthsApi>,
    payee_locations_api: Box<crate::apis::PayeeLocationsApi>,
    payees_api: Box<crate::apis::PayeesApi>,
    scheduled_transactions_api: Box<crate::apis::ScheduledTransactionsApi>,
    transactions_api: Box<crate::apis::TransactionsApi>,
    user_api: Box<crate::apis::UserApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            accounts_api: Box::new(crate::apis::AccountsApiClient::new(rc.clone())),
            budgets_api: Box::new(crate::apis::BudgetsApiClient::new(rc.clone())),
            categories_api: Box::new(crate::apis::CategoriesApiClient::new(rc.clone())),
            deprecated_api: Box::new(crate::apis::DeprecatedApiClient::new(rc.clone())),
            months_api: Box::new(crate::apis::MonthsApiClient::new(rc.clone())),
            payee_locations_api: Box::new(crate::apis::PayeeLocationsApiClient::new(rc.clone())),
            payees_api: Box::new(crate::apis::PayeesApiClient::new(rc.clone())),
            scheduled_transactions_api: Box::new(crate::apis::ScheduledTransactionsApiClient::new(rc.clone())),
            transactions_api: Box::new(crate::apis::TransactionsApiClient::new(rc.clone())),
            user_api: Box::new(crate::apis::UserApiClient::new(rc.clone())),
        }
    }

    pub fn accounts_api(&self) -> &crate::apis::AccountsApi{
        self.accounts_api.as_ref()
    }

    pub fn budgets_api(&self) -> &crate::apis::BudgetsApi{
        self.budgets_api.as_ref()
    }

    pub fn categories_api(&self) -> &crate::apis::CategoriesApi{
        self.categories_api.as_ref()
    }

    pub fn deprecated_api(&self) -> &crate::apis::DeprecatedApi{
        self.deprecated_api.as_ref()
    }

    pub fn months_api(&self) -> &crate::apis::MonthsApi{
        self.months_api.as_ref()
    }

    pub fn payee_locations_api(&self) -> &crate::apis::PayeeLocationsApi{
        self.payee_locations_api.as_ref()
    }

    pub fn payees_api(&self) -> &crate::apis::PayeesApi{
        self.payees_api.as_ref()
    }

    pub fn scheduled_transactions_api(&self) -> &crate::apis::ScheduledTransactionsApi{
        self.scheduled_transactions_api.as_ref()
    }

    pub fn transactions_api(&self) -> &crate::apis::TransactionsApi{
        self.transactions_api.as_ref()
    }

    pub fn user_api(&self) -> &crate::apis::UserApi{
        self.user_api.as_ref()
    }

}
