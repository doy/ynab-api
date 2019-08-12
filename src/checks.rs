pub fn run_checks(budget: &super::ynab::Budget) {
    check_reconciled(budget);
    check_has_inflows(budget);
}

fn check_reconciled(budget: &super::ynab::Budget) {
    let reconciled_amount: i64 = budget
        .reimbursables()
        .iter()
        .filter(|t| t.reimbursed)
        .map(|t| t.amount)
        .sum();
    if reconciled_amount != 0 {
        eprintln!(
            "reconciled reimbursables don't sum to $0.00: ${}",
            super::ynab::format_amount(reconciled_amount)
        );
        std::process::exit(1);
    }
}

fn check_has_inflows(budget: &super::ynab::Budget) {
    let txns = budget
        .reimbursables()
        .iter()
        .filter(|t| !t.reimbursed && t.amount > 0)
        .count();
    if txns == 0 {
        eprintln!("no transactions to reconcile");
        std::process::exit(1);
    }
}
