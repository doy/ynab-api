pub struct Client {
    api: ynab_api::apis::client::APIClient,
}

impl Client {
    pub fn new(key: &str) -> Self {
        let mut ynab_config =
            ynab_api::apis::configuration::Configuration::new();
        ynab_config.api_key = Some(ynab_api::apis::configuration::ApiKey {
            prefix: Some("Bearer".to_string()),
            key: key.to_string(),
        });
        Self {
            api: ynab_api::apis::client::APIClient::new(ynab_config),
        }
    }

    pub fn default_budget(&self) -> ynab_api::models::BudgetDetail {
        let budgets =
            self.api.budgets_api().get_budgets().unwrap().data.budgets;
        let budget = budgets.iter().next().unwrap();
        self.api
            .budgets_api()
            .get_budget_by_id(&budget.id, 0)
            .unwrap()
            .data
            .budget
    }

    pub fn update_transactions(
        &self,
        budget_id: &str,
        transactions: ynab_api::models::UpdateTransactionsWrapper,
    ) -> Option<String> {
        let res = self
            .api
            .transactions_api()
            .update_transactions(budget_id, transactions);
        if let Err(e) = res {
            Some(format!("{:?}", e))
        } else {
            None
        }
    }
}
