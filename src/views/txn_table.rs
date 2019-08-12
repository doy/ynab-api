use cursive::view::{Identifiable, ViewWrapper};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum TxnColumn {
    Selected,
    Date,
    Payee,
    Amount,
}

type TxnTableView =
    cursive_table_view::TableView<crate::ynab::Transaction, TxnColumn>;
pub struct TableView {
    view: cursive::views::OnEventView<cursive::views::IdView<TxnTableView>>,
}

impl cursive::view::ViewWrapper for TableView {
    cursive::wrap_impl!(
        self.view:
            cursive::views::OnEventView<cursive::views::IdView<TxnTableView>>
    );
}

impl TableView {
    pub fn len(&self) -> usize {
        self.view.get_inner().with_view(|v| v.len()).unwrap()
    }
}

impl cursive_table_view::TableViewItem<TxnColumn>
    for crate::ynab::Transaction
{
    fn to_column(&self, column: TxnColumn) -> String {
        match column {
            TxnColumn::Selected => {
                if self.selected {
                    "[x]".to_string()
                } else {
                    "[ ]".to_string()
                }
            }
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
            TxnColumn::Selected => std::cmp::Ordering::Equal,
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
    txn_table(inflows, "inflows_table")
}

pub fn outflows_table(budget: &crate::ynab::Budget) -> TableView {
    let outflows = budget
        .reimbursables()
        .iter()
        .filter(|t| !t.reimbursed && t.amount <= 0)
        .cloned()
        .collect();
    txn_table(outflows, "outflows_table")
}

fn txn_table(
    txns: Vec<crate::ynab::Transaction>,
    id: &'static str,
) -> TableView {
    let mut table = cursive_table_view::TableView::new()
        .column(TxnColumn::Selected, "Sel", |c| c.width(3))
        .column(TxnColumn::Date, "Date", |c| c.width(10))
        .column(TxnColumn::Payee, "Payee", |c| c)
        .column(TxnColumn::Amount, "Amount", |c| {
            c.align(cursive::align::HAlign::Right).width(10)
        })
        .default_column(TxnColumn::Date)
        .with_id(id);
    table.get_mut().set_items(txns);
    let view =
        cursive::views::OnEventView::new(table).on_event(' ', move |s| {
            s.call_on(
                &cursive::view::Selector::Id(id),
                |v: &mut TxnTableView| {
                    if let Some(idx) = v.item() {
                        let txn = v.borrow_item_mut(idx).unwrap();
                        txn.selected = !txn.selected;
                    }
                },
            );
        });
    TableView { view }
}
