pub struct Budget {
    client: super::client::Client,
    budget: ynab_api::models::BudgetDetail,
    reimbursables: Vec<super::transaction::Transaction>,
}

impl Budget {
    pub fn new(
        client: super::client::Client,
        budget: ynab_api::models::BudgetDetail,
    ) -> Self {
        let reimbursables = Self::get_reimbursables(&budget);
        Self {
            client,
            budget,
            reimbursables,
        }
    }

    pub fn name(&self) -> String {
        self.budget.name.clone()
    }

    pub fn id(&self) -> String {
        self.budget.id.clone()
    }

    pub fn reimbursables(&self) -> &[super::transaction::Transaction] {
        &self.reimbursables
    }

    pub fn reconcile_transactions(
        &self,
        txns: &[&super::transaction::Transaction],
    ) -> Option<String> {
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
        self.client.update_transactions(&self.budget.id, to_update)
    }

    fn get_reimbursables(
        budget: &ynab_api::models::BudgetDetail,
    ) -> Vec<super::transaction::Transaction> {
        let reimbursables_id = if let Some(categories) = &budget.categories {
            categories
                .iter()
                .find(|c| c.name == "Reimbursables")
                .map(|c| c.id.clone())
                .unwrap()
        } else {
            panic!("no categories found")
        };

        let mut reimbursables = vec![];
        if let Some(payees) = &budget.payees {
            let mut payee_map = std::collections::HashMap::new();
            for p in payees {
                payee_map.insert(p.id.clone(), p.name.clone());
            }
            let payee_map = payee_map;

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

                    let mut txn =
                        super::transaction::Transaction::from_transaction(t);
                    txn.payee = payee;
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
                                .map(|payee_id| {
                                    payee_map.get(payee_id).cloned()
                                })
                                .next()
                                .unwrap_or(None)
                        });

                    let mut txn =
                        super::transaction::Transaction::from_sub_transaction(
                            t, st,
                        );
                    txn.payee = payee;
                    reimbursables.push(txn);
                }
            }
        } else {
            panic!("no payees?");
        }

        reimbursables.sort_by_cached_key(|t| t.date.clone());
        reimbursables
    }
}
