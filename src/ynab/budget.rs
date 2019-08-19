use snafu::{OptionExt, ResultExt};

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    #[snafu(display("couldn't get default budget: {}", source))]
    GetBudget { source: super::client::Error },

    #[snafu(display("couldn't update transactions: {}", source))]
    UpdateTransactions { source: super::client::Error },

    #[snafu(display("couldn't find the reimbursables category"))]
    FindReimbursablesCategory,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Budget {
    client: super::client::Client,
    id: String,
    name: String,
    reimbursables: Vec<super::transaction::Transaction>,
}

impl Budget {
    pub fn new(key: &str) -> Result<Self> {
        let client = super::client::Client::new(key);
        let budget = client.default_budget().context(GetBudget)?;
        let reimbursables = Self::get_reimbursables(&budget)?;
        let budget = Self {
            client,
            id: budget.id.clone(),
            name: budget.name.clone(),
            reimbursables,
        };
        budget.check();
        Ok(budget)
    }

    #[must_use]
    pub fn refresh(&mut self) -> Result<()> {
        let budget = self.client.default_budget().context(GetBudget)?;
        self.id = budget.id.clone();
        self.name = budget.name.clone();
        self.reimbursables = Self::get_reimbursables(&budget)?;
        self.check();
        Ok(())
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn reimbursables(&self) -> &[super::transaction::Transaction] {
        &self.reimbursables
    }

    #[must_use]
    pub fn reconcile_transactions(
        &self,
        txns: &[&super::transaction::Transaction],
    ) -> Result<()> {
        let mut to_update =
            ynab_api::models::UpdateTransactionsWrapper::new();
        to_update.transactions = Some(
            txns.iter()
                .map(|t| {
                    let mut ut = t.to_update_transaction();
                    ut.flag_color = Some("green".to_string());
                    ut
                })
                .collect(),
        );
        self.client
            .update_transactions(&self.id, to_update)
            .context(UpdateTransactions)?;
        Ok(())
    }

    fn get_reimbursables(
        budget: &ynab_api::models::BudgetDetail,
    ) -> Result<Vec<super::transaction::Transaction>> {
        let reimbursables_id = budget
            .categories
            .as_ref()
            .and_then(|categories| {
                categories
                    .iter()
                    .find(|c| c.name == "Reimbursables")
                    .map(|c| c.id.clone())
            })
            .context(FindReimbursablesCategory)?;

        let mut payee_map = std::collections::HashMap::new();
        if let Some(payees) = &budget.payees {
            for p in payees {
                payee_map.insert(p.id.clone(), p.name.clone());
            }
        }
        let payee_map = payee_map;

        let mut account_map = std::collections::HashMap::new();
        if let Some(accounts) = &budget.accounts {
            for a in accounts {
                account_map.insert(a.id.clone(), a.name.clone());
            }
        }
        let account_map = account_map;

        let mut reimbursables = vec![];

        let mut transaction_map = std::collections::HashMap::new();
        if let Some(transactions) = &budget.transactions {
            for t in transactions {
                transaction_map.insert(t.id.clone(), t);

                if let Some(category_id) = &t.category_id {
                    if category_id != &reimbursables_id {
                        continue;
                    }
                } else {
                    continue;
                }

                let payee = t
                    .payee_id
                    .iter()
                    .map(|payee_id| payee_map.get(payee_id).cloned())
                    .next()
                    .unwrap_or(None);
                let account = account_map.get(&t.account_id).cloned();

                let mut txn =
                    super::transaction::Transaction::from_transaction(t);
                txn.payee = payee;
                txn.account = account;
                reimbursables.push(txn);
            }
        }
        let transaction_map = transaction_map;

        if let Some(subtransactions) = &budget.subtransactions {
            for st in subtransactions {
                if let Some(category_id) = &st.category_id {
                    if category_id != &reimbursables_id {
                        continue;
                    }
                } else {
                    continue;
                }

                let t = transaction_map[&st.transaction_id];
                let payee = st
                    .payee_id
                    .iter()
                    .map(|payee_id| payee_map.get(payee_id).cloned())
                    .next()
                    .unwrap_or_else(|| {
                        t.payee_id
                            .iter()
                            .map(|payee_id| payee_map.get(payee_id).cloned())
                            .next()
                            .unwrap_or(None)
                    });
                let account = account_map.get(&t.account_id).cloned();

                let mut txn =
                    super::transaction::Transaction::from_sub_transaction(
                        t, st,
                    );
                txn.payee = payee;
                txn.account = account;
                reimbursables.push(txn);
            }
        }

        reimbursables.sort_by_cached_key(|t| t.date.clone());
        Ok(reimbursables)
    }

    fn check(&self) {
        self.check_reconciled();
        self.check_has_inflows();
    }

    fn check_reconciled(&self) {
        let reconciled_amount: i64 = self
            .reimbursables()
            .iter()
            .filter(|t| t.reimbursed)
            .map(|t| t.amount)
            .sum();
        if reconciled_amount != 0 {
            eprintln!(
                "reconciled reimbursables don't sum to $0.00: ${}",
                crate::ynab::format_amount(reconciled_amount)
            );
            std::process::exit(1);
        }
    }

    fn check_has_inflows(&self) {
        let txns = self
            .reimbursables()
            .iter()
            .filter(|t| !t.reimbursed && t.amount > 0)
            .count();
        if txns == 0 {
            eprintln!("no transactions to reconcile");
            std::process::exit(1);
        }
    }
}
