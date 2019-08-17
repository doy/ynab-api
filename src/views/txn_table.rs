use cursive::view::{Identifiable, ViewWrapper};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum TxnColumn {
    Selected,
    Date,
    Account,
    Payee,
    Amount,
    TotalAmount,
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
        .column(TxnColumn::Account, "Account", |c| c.width(15))
        .column(TxnColumn::Payee, "Payee", |c| c)
        .column(TxnColumn::Amount, "Amount", |c| {
            c.align(cursive::align::HAlign::Right).width(10)
        })
        .column(TxnColumn::TotalAmount, "", |c| {
            c.align(cursive::align::HAlign::Right).width(10)
        })
        .default_column(TxnColumn::Date)
        .on_submit(move |s, _, _| {
            let outflows: Vec<_> = s
                .call_on_id("outflows_table", |v: &mut TxnTableView| {
                    v.borrow_items()
                        .iter()
                        .filter(|t| t.selected)
                        .cloned()
                        .collect()
                })
                .unwrap();
            let inflows: Vec<_> = s
                .call_on_id("inflows_table", |v: &mut TxnTableView| {
                    v.borrow_items()
                        .iter()
                        .filter(|t| t.selected)
                        .cloned()
                        .collect()
                })
                .unwrap();
            let total_outflow: i64 = outflows.iter().map(|t| t.amount).sum();
            let total_inflow: i64 = inflows.iter().map(|t| t.amount).sum();
            let total_amount = total_outflow + total_inflow;
            if total_amount == 0 {
                let budget: &mut crate::ynab::Budget = s.user_data().unwrap();
                let txns: Vec<_> =
                    outflows.iter().chain(inflows.iter()).collect();
                let err = budget.reconcile_transactions(&txns);
                if let Some(err) = err {
                    s.add_layer(dialog(&format!("Error: {}", err)))
                } else {
                    s.add_layer(dialog(&format!(
                        "Successfully updated {} transactions",
                        txns.len()
                    )));
                    s.call_on_id("outflows_table", |v: &mut TxnTableView| {
                        let all_txns = v.borrow_items_mut();
                        for id in txns.iter().map(|t| t.id.clone()) {
                            if let Some(idx) =
                                all_txns.iter().position(|t| t.id == id)
                            {
                                all_txns.remove(idx);
                            }
                        }
                        if let Some(row) = v.row() {
                            if row >= v.len() {
                                v.set_selected_row(v.len() - 1);
                            }
                        }
                    })
                    .unwrap();
                    s.call_on_id("inflows_table", |v: &mut TxnTableView| {
                        let all_txns = v.borrow_items_mut();
                        for id in txns.iter().map(|t| t.id.clone()) {
                            if let Some(idx) =
                                all_txns.iter().position(|t| t.id == id)
                            {
                                all_txns.remove(idx);
                            }
                        }
                        if let Some(row) = v.row() {
                            if row >= v.len() {
                                v.set_selected_row(v.len() - 1);
                            }
                        }
                    })
                    .unwrap();
                }
            } else {
                s.add_layer(dialog(&format!(
                    "Selected amount is {}, must be 0",
                    crate::ynab::format_amount(total_amount)
                )))
            }
        })
        .with_id(id);
    table.get_mut().set_items(txns);
    let view = cursive::views::OnEventView::new(table)
        .on_event(' ', move |s| {
            s.call_on_id(&id, |v: &mut TxnTableView| {
                if let Some(idx) = v.item() {
                    let txn = v.borrow_item_mut(idx).unwrap();
                    txn.selected = !txn.selected;
                }
            });
            render_selected_total(s);
        })
        .on_event('h', |s| {
            s.on_event(cursive::event::Event::Key(cursive::event::Key::Left))
        })
        .on_event('j', |s| {
            s.on_event(cursive::event::Event::Key(cursive::event::Key::Down))
        })
        .on_event('k', |s| {
            s.on_event(cursive::event::Event::Key(cursive::event::Key::Up))
        })
        .on_event('l', |s| {
            s.on_event(cursive::event::Event::Key(cursive::event::Key::Right))
        })
        .on_event('g', move |s| {
            s.call_on_id(&id, |v: &mut TxnTableView| {
                v.set_selected_row(0);
            })
            .unwrap();
        })
        .on_event('G', move |s| {
            s.call_on_id(&id, |v: &mut TxnTableView| {
                v.set_selected_row(v.len() - 1);
            })
            .unwrap();
        })
        .on_event('r', move |s| {
            let budget: &mut crate::ynab::Budget = s.user_data().unwrap();
            budget.refresh();

            let mut inflows: Vec<_> = budget
                .reimbursables()
                .iter()
                .filter(|t| !t.reimbursed && t.amount > 0)
                .cloned()
                .collect();
            s.call_on_id("inflows_table", |v: &mut TxnTableView| {
                let selected: std::collections::HashSet<_> = v
                    .borrow_items()
                    .iter()
                    .filter(|t| t.selected)
                    .map(|t| t.id.clone())
                    .collect();
                let row = v
                    .item()
                    .and_then(|idx| v.borrow_item(idx).map(|t| t.id.clone()));
                for mut t in inflows.iter_mut() {
                    if selected.contains(&t.id) {
                        t.selected = true;
                    }
                }
                let idx = row
                    .and_then(|id| inflows.iter().position(|t| t.id == id));
                v.set_items(inflows);
                if let Some(idx) = idx {
                    v.set_selected_item(idx);
                }
            })
            .unwrap();

            let budget: &mut crate::ynab::Budget = s.user_data().unwrap();
            let mut outflows: Vec<_> = budget
                .reimbursables()
                .iter()
                .filter(|t| !t.reimbursed && t.amount <= 0)
                .cloned()
                .collect();
            s.call_on_id("outflows_table", |v: &mut TxnTableView| {
                let selected: std::collections::HashSet<_> = v
                    .borrow_items()
                    .iter()
                    .filter(|t| t.selected)
                    .map(|t| t.id.clone())
                    .collect();
                let row = v
                    .item()
                    .and_then(|idx| v.borrow_item(idx).map(|t| t.id.clone()));
                for mut t in outflows.iter_mut() {
                    if selected.contains(&t.id) {
                        t.selected = true;
                    }
                }
                let idx = row
                    .and_then(|id| outflows.iter().position(|t| t.id == id));
                v.set_items(outflows);
                if let Some(idx) = idx {
                    v.set_selected_item(idx);
                }
            })
            .unwrap();

            render_selected_total(s);
        });
    TableView { view }
}

pub fn txn_tables(budget: &crate::ynab::Budget) -> impl cursive::view::View {
    let mut layout = cursive::views::LinearLayout::vertical();

    layout.add_child(
        cursive::views::TextView::new("Selected: $0.00 (0 transactions)")
            .h_align(cursive::align::HAlign::Right)
            .with_id("selected_total"),
    );

    let mut inflows_table = inflows_table(&budget);
    layout.add_child(cursive::views::TextView::new(format!(
        "Inflows: {} ({} transaction{}",
        crate::ynab::format_amount(inflows_table.amount()),
        inflows_table.len(),
        if inflows_table.len() == 1 { ") " } else { "s)" }
    )));
    layout.add_child(cursive::views::CircularFocus::wrap_arrows(
        cursive::views::BoxView::with_min_height(
            std::cmp::min(std::cmp::max(inflows_table.len(), 1), 5) + 2,
            cursive::views::BoxView::with_full_width(inflows_table),
        ),
    ));

    layout.add_child(cursive::views::TextView::new(" "));

    let mut outflows_table = outflows_table(&budget);
    layout.add_child(cursive::views::TextView::new(format!(
        "Outflows: {} ({} transaction{}",
        crate::ynab::format_amount(outflows_table.amount()),
        outflows_table.len(),
        if outflows_table.len() == 1 {
            ") "
        } else {
            "s)"
        }
    )));
    layout.add_child(cursive::views::CircularFocus::wrap_arrows(
        cursive::views::BoxView::with_full_screen(outflows_table),
    ));

    layout
}

fn render_selected_total(s: &mut cursive::Cursive) {
    let outflows: Vec<_> = s
        .call_on_id("outflows_table", |v: &mut TxnTableView| {
            v.borrow_items()
                .iter()
                .filter(|t| t.selected)
                .map(|t| t.amount)
                .collect()
        })
        .unwrap();
    let inflows: Vec<_> = s
        .call_on_id("inflows_table", |v: &mut TxnTableView| {
            v.borrow_items()
                .iter()
                .filter(|t| t.selected)
                .map(|t| t.amount)
                .collect()
        })
        .unwrap();
    let outflow: i64 = outflows.iter().sum();
    let inflow: i64 = inflows.iter().sum();
    let amount = outflow + inflow;
    s.call_on_id("selected_total", |v: &mut cursive::views::TextView| {
        let mut sstr =
            cursive::utils::markup::StyledString::plain("Selected: ");
        let color = if amount == 0 && outflows.len() + inflows.len() != 0 {
            cursive::theme::Color::Dark(cursive::theme::BaseColor::Green)
        } else {
            cursive::theme::Color::TerminalDefault
        };
        sstr.append(cursive::utils::markup::StyledString::styled(
            crate::ynab::format_amount(amount),
            color,
        ));
        sstr.append(format!(
            " ({} transaction{}",
            outflows.len() + inflows.len(),
            if outflows.len() + inflows.len() == 1 {
                ") "
            } else {
                "s)"
            }
        ));
        v.set_content(sstr);
    });
}

fn dialog(s: &str) -> impl cursive::view::View {
    cursive::views::Panel::new(cursive::views::Dialog::info(s))
}
