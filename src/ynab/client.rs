#[derive(Debug, snafu::Snafu)]
pub enum Error {
    // ynab-api error types don't implement Error, so can't use the
    // auto-source behavior
    #[snafu(display("failed to update transactions: {}", source_msg))]
    UpdateTransactions { source_msg: String },

    #[snafu(display("failed to get budgets: {}", source_msg))]
    GetBudgets { source_msg: String },

    #[snafu(display("failed to get budget {}: {}", id, source_msg))]
    GetBudgetById { id: String, source_msg: String },
}

pub type Result<T> = std::result::Result<T, Error>;

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

    pub fn default_budget(&self) -> Result<ynab_api::models::BudgetDetail> {
        let budget_id = self
            .api
            .budgets_api()
            .get_budgets()
            .map_err(|e| Error::GetBudgets {
                source_msg: format!("{:?}", e),
            })?
            .data
            .budgets
            .iter()
            .next()
            .ok_or_else(|| Error::GetBudgets {
                source_msg: "no budgets found".to_string(),
            })?
            .id
            .clone();
        Ok(self
            .api
            .budgets_api()
            .get_budget_by_id(&budget_id, 0)
            .map_err(|e| Error::GetBudgetById {
                id: budget_id.clone(),
                source_msg: format!("{:?}", e),
            })?
            .data
            .budget)
    }

    pub fn update_transactions(
        &self,
        budget_id: &str,
        transactions: ynab_api::models::UpdateTransactionsWrapper,
    ) -> Result<()> {
        self.api
            .transactions_api()
            .update_transactions(budget_id, transactions)
            .map(|_| ())
            .map_err(|e| Error::UpdateTransactions {
                source_msg: format!("{:?}", e),
            })
    }
}
