mod ynab;

fn main() {
    let key = std::env::args().nth(1).unwrap();
    let client = ynab::Client::new(&key);
    let budget = client.default_budget();
    println!("using budget {} ({})", budget.name(), budget.id());

    for t in budget.reimbursables() {
        if t.reimbursed {
            continue;
        }

        println!(
            "{} | {} | {}",
            t.date,
            t.payee,
            ynab::format_amount(t.amount)
        )
    }
}
