#[derive(Clone, Debug)]
pub struct Transaction {
    pub date: String,
    pub payee: String,
    pub amount: i64,
    pub reimbursed: bool,
}
