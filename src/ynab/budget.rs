pub struct Budget<'a> {
    api: &'a ynab_api::apis::client::APIClient,
    budget: ynab_api::models::BudgetDetail,
}

impl<'a> Budget<'a> {
    pub fn new(
        api: &'a ynab_api::apis::client::APIClient,
        budget: ynab_api::models::BudgetDetail,
    ) -> Self {
        Self { api, budget }
    }

    pub fn name(&self) -> String {
        self.budget.name.clone()
    }

    pub fn id(&self) -> String {
        self.budget.id.clone()
    }

    pub fn reimbursables(&self) -> Vec<super::transaction::Transaction> {
        let reimbursables_id = self
            .api
            .categories_api()
            .get_categories(&self.budget.id, 0)
            .unwrap()
            .data
            .category_groups
            .iter()
            .map(|group| {
                group
                    .categories
                    .iter()
                    .map(|c| (c.id.clone(), c.name.clone()))
            })
            .flat_map(|cs| cs)
            .find(|(_, name)| name == "Reimbursables")
            .map(|(id, _)| id)
            .unwrap();

        let mut reimbursables = vec![];
        if let Some(transactions) = &self.budget.transactions {
            if let Some(payees) = &self.budget.payees {
                let mut payee_map = std::collections::HashMap::new();
                for p in payees {
                    payee_map.insert(p.id.clone(), p.name.clone());
                }
                let payee_map = payee_map;

                for t in transactions {
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
                        .flat_map(|payee_id| payee_map.get(payee_id).cloned())
                        .next()
                        .unwrap_or_else(|| "(none)".to_string());
                    let reimbursed = if let Some(color) = &t.flag_color {
                        color == "green"
                    } else {
                        false
                    };

                    reimbursables.push(super::transaction::Transaction {
                        date: t.date.clone(),
                        payee,
                        amount: t.amount,
                        reimbursed,
                    })
                }
            } else {
                panic!("no payees?");
            }
        }

        reimbursables
    }
}
