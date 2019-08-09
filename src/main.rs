fn client(key: &str) -> ynab_api::apis::client::APIClient {
    let mut ynab_config = ynab_api::apis::configuration::Configuration::new();
    ynab_config.api_key = Some(ynab_api::apis::configuration::ApiKey {
        prefix: Some("Bearer".to_string()),
        key: key.to_string(),
    });
    ynab_api::apis::client::APIClient::new(ynab_config)
}

fn format_amount(amount: i64) -> String {
    let dollars = amount.abs() / 1000;
    let cents = (amount.abs() % 1000) / 10;
    let sign = if amount < 0 { "-" } else { "" };
    format!("{}{}.{:02}", sign, dollars, cents)
}

fn main() {
    let key = std::env::args().nth(1).unwrap();
    let client = client(&key);

    let budgets = client.budgets_api().get_budgets().unwrap().data.budgets;
    let budget = budgets.iter().next().unwrap();
    println!("budget is {} ({})", budget.name, budget.id);

    let reimbursables_id = client
        .categories_api()
        .get_categories(&budget.id, 0)
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
    println!("found reimbursables category: {}", reimbursables_id);

    let full_budget = client
        .budgets_api()
        .get_budget_by_id(&budget.id, 0)
        .unwrap()
        .data
        .budget;
    println!("got full budget");

    let transactions = full_budget.transactions.unwrap();
    let payees = full_budget.payees.unwrap();

    for t in transactions {
        if let Some(category_id) = t.category_id {
            if category_id != reimbursables_id {
                continue;
            }
        } else {
            continue;
        }

        if t.flag_color.is_some() {
            continue;
        }

        let payee = t
            .payee_id
            .map(|id| {
                payees.iter().find(|p| p.id == id).unwrap().name.clone()
            })
            .unwrap_or_else(|| "(none)".to_string());
        println!("{} | {} | {}", t.date, payee, format_amount(t.amount))
    }
}
