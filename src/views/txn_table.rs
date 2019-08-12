#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum TxnColumn {
    Date,
    Payee,
    Amount,
}

type TableView =
    cursive_table_view::TableView<crate::ynab::Transaction, TxnColumn>;

impl cursive_table_view::TableViewItem<TxnColumn>
    for crate::ynab::Transaction
{
    fn to_column(&self, column: TxnColumn) -> String {
        match column {
            TxnColumn::Date => self.date.clone(),
            TxnColumn::Payee => self.payee.clone(),
            TxnColumn::Amount => crate::ynab::format_amount(self.amount),
        }
    }

    fn cmp(&self, other: &Self, column: TxnColumn) -> std::cmp::Ordering
    where
        Self: Sized,
    {
        match column {
            TxnColumn::Date => self.date.cmp(&other.date),
            TxnColumn::Payee => self.payee.cmp(&other.payee),
            TxnColumn::Amount => self.amount.cmp(&other.amount),
        }
    }
}

pub fn inflows_table(budget: &crate::ynab::Budget) -> TableView {
    let inflows = budget
        .reimbursables()
        .iter()
        .filter(|t| !t.reimbursed && t.amount > 0)
        .cloned()
        .collect();
    txn_table(inflows)
}

pub fn outflows_table(budget: &crate::ynab::Budget) -> TableView {
    let outflows = budget
        .reimbursables()
        .iter()
        .filter(|t| !t.reimbursed && t.amount <= 0)
        .cloned()
        .collect();
    txn_table(outflows)
}

fn txn_table(txns: Vec<crate::ynab::Transaction>) -> TableView {
    let mut table = cursive_table_view::TableView::new()
        .column(TxnColumn::Date, "Date", |c| c.width(10))
        .column(TxnColumn::Payee, "Payee", |c| c)
        .column(TxnColumn::Amount, "Amount", |c| {
            c.align(cursive::align::HAlign::Right).width(10)
        })
        .default_column(TxnColumn::Date);
    table.set_items(txns);
    table
}
