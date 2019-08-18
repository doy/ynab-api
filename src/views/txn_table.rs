use cursive::view::{Identifiable, View, ViewWrapper};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum TxnColumn {
    Selected,
    Date,
    Account,
    Payee,
    Amount,
    TotalAmount,
}

pub type TxnTableView =
    cursive_table_view::TableView<crate::ynab::Transaction, TxnColumn>;
pub struct TxnTable {
    view: super::util::FullView<TxnTableView>,
}

impl cursive::view::ViewWrapper for TxnTable {
    cursive::wrap_impl!(self.view: super::util::FullView<TxnTableView>);
}

impl TxnTable {
    pub fn new(
        txns: Vec<crate::ynab::Transaction>,
        id: &'static str,
    ) -> Self {
        let mut table = cursive_table_view::TableView::new()
            .column(TxnColumn::Selected, "Sel", |c| c.width(3))
            .column(TxnColumn::Date, "Date", |c| c.width(10))
            .column(TxnColumn::Account, "Account", |c| c.width(15))
            .column(TxnColumn::Payee, "Payee", |c| c)
            .column(TxnColumn::Amount, "Amount", |c| {
                c.align(cursive::align::HAlign::Right).width(10)
            })
            .column(TxnColumn::TotalAmount, "", |c| {
                c.align(cursive::align::HAlign::Right).width(10)
            })
            .default_column(TxnColumn::Date)
            .on_submit(|s, _, _| {
                s.on_event(cursive::event::Event::Key(
                    cursive::event::Key::F0,
                ));
            });
        table.set_items(txns);
        let view = cursive::views::OnEventView::new(table)
            .on_event_inner(' ', |v: &mut TxnTableView, _| {
                if let Some(idx) = v.item() {
                    let txn = v.borrow_item_mut(idx).unwrap();
                    txn.selected = !txn.selected;
                }
                None
            })
            .on_event_inner('h', |v: &mut TxnTableView, _| {
                v.on_event(cursive::event::Event::Key(
                    cursive::event::Key::Left,
                ));
                None
            })
            .on_event_inner('j', |v: &mut TxnTableView, _| {
                v.on_event(cursive::event::Event::Key(
                    cursive::event::Key::Down,
                ));
                None
            })
            .on_event_inner('k', |v: &mut TxnTableView, _| {
                v.on_event(cursive::event::Event::Key(
                    cursive::event::Key::Up,
                ));
                None
            })
            .on_event_inner('l', |v: &mut TxnTableView, _| {
                v.on_event(cursive::event::Event::Key(
                    cursive::event::Key::Right,
                ));
                None
            })
            .on_event_inner('g', |v: &mut TxnTableView, _| {
                v.set_selected_row(0);
                None
            })
            .on_event_inner('G', |v: &mut TxnTableView, _| {
                v.set_selected_row(v.len() - 1);
                None
            })
            .with_id(id);
        TxnTable { view }
    }

    pub fn len(&self) -> usize {
        self.view
            .with_view(|v| v.with_view(|v| v.len()).unwrap())
            .unwrap()
    }

    // XXX why does borrow_items require &mut self?
    pub fn amount(&mut self) -> i64 {
        self.view
            .get_mut()
            .get_inner_mut()
            .borrow_items()
            .iter()
            .map(|t| t.amount)
            .sum()
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
            TxnColumn::Account => {
                self.account.clone().unwrap_or_else(|| "".to_string())
            }
            TxnColumn::Payee => {
                self.payee.clone().unwrap_or_else(|| "".to_string())
            }
            TxnColumn::Amount => crate::ynab::format_amount(self.amount),
            TxnColumn::TotalAmount => {
                if self.amount == self.total_amount {
                    "".to_string()
                } else {
                    crate::ynab::format_amount(self.total_amount)
                }
            }
        }
    }

    fn cmp(&self, other: &Self, column: TxnColumn) -> std::cmp::Ordering
    where
        Self: Sized,
    {
        match column {
            TxnColumn::Selected => std::cmp::Ordering::Equal,
            TxnColumn::Date => self.date.cmp(&other.date),
            TxnColumn::Account => self.account.cmp(&other.account),
            TxnColumn::Payee => self.payee.cmp(&other.payee),
            TxnColumn::Amount => self.amount.cmp(&other.amount),
            TxnColumn::TotalAmount => self.amount.cmp(&other.total_amount),
        }
    }
}
