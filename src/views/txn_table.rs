use cursive::view::{Identifiable, ViewWrapper};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum TxnColumn {
    Selected,
    Date,
    Payee,
    Amount,
}

type TxnTableView =
    cursive_table_view::TableView<crate::ynab::Transaction, TxnColumn>;
struct TableView {
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

    // XXX why does borrow_items require &mut self?
    pub fn amount(&mut self) -> i64 {
        self.view
            .get_inner_mut()
            .get_mut()
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

fn inflows_table(budget: &crate::ynab::Budget) -> TableView {
    let inflows = budget
        .reimbursables()
        .iter()
        .filter(|t| !t.reimbursed && t.amount > 0)
        .cloned()
        .collect();
    txn_table(inflows, "inflows_table")
}

fn outflows_table(budget: &crate::ynab::Budget) -> TableView {
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
        .on_submit(move |s, _, _| {
            let total_outflow = s
                .call_on_id("outflows_table", |v: &mut TxnTableView| -> i64 {
                    v.borrow_items()
                        .iter()
                        .filter(|t| t.selected)
                        .map(|t| t.amount)
                        .sum()
                })
                .unwrap();
            let total_inflow = s
                .call_on_id("inflows_table", |v: &mut TxnTableView| -> i64 {
                    v.borrow_items()
                        .iter()
                        .filter(|t| t.selected)
                        .map(|t| t.amount)
                        .sum()
                })
                .unwrap();
            let total_amount = total_outflow + total_inflow;
            if total_amount == 0 {
                s.add_layer(cursive::views::Dialog::info(
                    "success, sum is zero!",
                ))
            } else {
                s.add_layer(cursive::views::Dialog::info(format!(
                    "failed, sum is {}",
                    total_amount
                )))
            }
        })
        .with_id(id);
    table.get_mut().set_items(txns);
    let view =
        cursive::views::OnEventView::new(table).on_event(' ', move |s| {
            s.call_on_id(&id, |v: &mut TxnTableView| {
                if let Some(idx) = v.item() {
                    let txn = v.borrow_item_mut(idx).unwrap();
                    txn.selected = !txn.selected;
                }
            });
        });
    TableView { view }
}

pub fn txn_tables(budget: &crate::ynab::Budget) -> impl cursive::view::View {
    let mut layout = cursive::views::LinearLayout::vertical();

    let mut inflows_table = inflows_table(&budget);
    layout.add_child(cursive::views::TextView::new(format!(
        "\nInflows: {} ({} transactions)",
        crate::ynab::format_amount(inflows_table.amount()),
        inflows_table.len()
    )));
    layout.add_child(crate::views::vi_view(
        cursive::views::CircularFocus::wrap_arrows(
            cursive::views::BoxView::with_min_height(
                std::cmp::min(std::cmp::max(inflows_table.len(), 1), 5) + 2,
                cursive::views::BoxView::with_full_width(inflows_table),
            ),
        ),
    ));

    let mut outflows_table = outflows_table(&budget);
    layout.add_child(cursive::views::TextView::new(format!(
        "\nOutflows: {} ({} transactions)",
        crate::ynab::format_amount(outflows_table.amount()),
        outflows_table.len()
    )));
    layout.add_child(crate::views::vi_view(
        cursive::views::CircularFocus::wrap_arrows(
            cursive::views::BoxView::with_full_screen(outflows_table),
        ),
    ));

    layout
}
