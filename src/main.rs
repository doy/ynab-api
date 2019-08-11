mod ynab;

fn main() {
    let key = std::env::args().nth(1).unwrap();
    let client = ynab::Client::new(&key);
    let budget = client.default_budget();
    let reimbursables = budget.reimbursables();
    println!("using budget {} ({})", budget.name(), budget.id());

    let reconciled_amount: i64 = reimbursables
        .iter()
        .filter(|t| t.reimbursed)
        .map(|t| t.amount)
        .sum();
    if reconciled_amount != 0 {
        eprintln!(
            "reconciled reimbursables don't sum to $0.00: ${}",
            ynab::format_amount(reconciled_amount)
        );
        std::process::exit(1);
    }
    println!("reconciled reimbursables correctly sum to $0.00");

    println!("ready for reconciliation:");
    for t in reimbursables
        .iter()
        .filter(|t| !t.reimbursed && t.amount > 0)
    {
        println!(
            "{} | {} | {}",
            t.date,
            t.payee,
            ynab::format_amount(t.amount)
        )
    }

    println!("match against reconcilable:");
    for t in reimbursables
        .iter()
        .filter(|t| !t.reimbursed && t.amount <= 0)
    {
        println!(
            "{} | {} | {}",
            t.date,
            t.payee,
            ynab::format_amount(t.amount)
        )
    }
}
