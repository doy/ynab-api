pub fn format_amount(amount: i64) -> String {
    let dollars = amount.abs() / 1000;
    let cents = (amount.abs() % 1000) / 10;
    let sign = if amount < 0 { "-" } else { "" };
    format!("${}{}.{:02}", sign, dollars, cents)
}
