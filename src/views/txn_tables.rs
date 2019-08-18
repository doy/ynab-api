use cursive::view::{Identifiable, View};

const SELECTED_TOTAL_ID: &str = "selected_total";
const INFLOWS_TABLE_ID: &str = "inflows_table";
const OUTFLOWS_TABLE_ID: &str = "outflows_table";

pub struct TxnTables {
    view: super::util::FullView<cursive::views::LinearLayout>,
}

impl cursive::view::ViewWrapper for TxnTables {
    cursive::wrap_impl!(
        self.view: super::util::FullView<cursive::views::LinearLayout>
    );
}

impl TxnTables {
    pub fn new(id: &'static str, budget: &crate::ynab::Budget) -> Self {
        let mut layout = cursive::views::LinearLayout::vertical();

        layout.add_child(
            cursive::views::TextView::new("Selected: $0.00 (0 transactions)")
                .h_align(cursive::align::HAlign::Right)
                .with_id(SELECTED_TOTAL_ID),
        );

        let inflows = budget
            .reimbursables()
            .iter()
            .filter(|t| !t.reimbursed && t.amount > 0)
            .cloned()
            .collect();
        let mut inflows_table =
            super::txn_table::TxnTable::new(inflows, INFLOWS_TABLE_ID);
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

        let outflows = budget
            .reimbursables()
            .iter()
            .filter(|t| !t.reimbursed && t.amount <= 0)
            .cloned()
            .collect();
        let mut outflows_table =
            super::txn_table::TxnTable::new(outflows, OUTFLOWS_TABLE_ID);
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

        let event_view = cursive::views::OnEventView::new(layout)
            .on_event(cursive::event::Key::F0, move |s| {
                submit(s);
            })
            .on_event('r', move |s| {
                refresh(s);
            })
            .on_pre_event_inner(' ', |v, _| select(v))
            .with_id(id);

        TxnTables { view: event_view }
    }
}

fn submit(s: &mut cursive::Cursive) {
    let inflows: Vec<_> = s
        .call_on_id(
            INFLOWS_TABLE_ID,
            |v: &mut cursive::views::OnEventView<
                super::txn_table::TxnTableView,
            >| {
                v.get_inner_mut()
                    .borrow_items()
                    .iter()
                    .filter(|t| t.selected)
                    .cloned()
                    .collect()
            },
        )
        .unwrap();
    let outflows: Vec<_> = s
        .call_on_id(
            OUTFLOWS_TABLE_ID,
            |v: &mut cursive::views::OnEventView<
                super::txn_table::TxnTableView,
            >| {
                v.get_inner_mut()
                    .borrow_items()
                    .iter()
                    .filter(|t| t.selected)
                    .cloned()
                    .collect()
            },
        )
        .unwrap();
    let total_inflow: i64 = inflows.iter().map(|t| t.amount).sum();
    let total_outflow: i64 = outflows.iter().map(|t| t.amount).sum();
    let total_amount = total_inflow + total_outflow;
    if total_amount == 0 && (inflows.len() + outflows.len() > 0) {
        let budget: &mut crate::ynab::Budget = s.user_data().unwrap();
        let txns: Vec<_> = inflows.iter().chain(outflows.iter()).collect();
        let err = budget.reconcile_transactions(&txns);
        if let Some(err) = err {
            s.add_layer(super::util::dialog(&format!("Error: {}", err)))
        } else {
            s.add_layer(super::util::dialog(&format!(
                "Successfully updated {} transactions",
                txns.len()
            )));
            s.call_on_id(
                INFLOWS_TABLE_ID,
                |v: &mut cursive::views::OnEventView<
                    super::txn_table::TxnTableView,
                >| {
                    let v = v.get_inner_mut();
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
                },
            )
            .unwrap();
            s.call_on_id(
                OUTFLOWS_TABLE_ID,
                |v: &mut cursive::views::OnEventView<
                    super::txn_table::TxnTableView,
                >| {
                    let v = v.get_inner_mut();
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
                },
            )
            .unwrap();
        }
    } else if total_amount != 0 {
        s.add_layer(super::util::dialog(&format!(
            "Selected amount is {}, must be 0",
            crate::ynab::format_amount(total_amount)
        )))
    }
}

fn refresh(s: &mut cursive::Cursive) {
    let budget: &mut crate::ynab::Budget = s.user_data().unwrap();
    budget.refresh();

    let mut inflows: Vec<_> = budget
        .reimbursables()
        .iter()
        .filter(|t| !t.reimbursed && t.amount > 0)
        .cloned()
        .collect();
    s.call_on_id(
        INFLOWS_TABLE_ID,
        |v: &mut cursive::views::OnEventView<
            super::txn_table::TxnTableView,
        >| {
            let v = v.get_inner_mut();
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
            let idx =
                row.and_then(|id| inflows.iter().position(|t| t.id == id));
            v.set_items(inflows);
            if let Some(idx) = idx {
                v.set_selected_item(idx);
            }
        },
    )
    .unwrap();

    let budget: &mut crate::ynab::Budget = s.user_data().unwrap();
    let mut outflows: Vec<_> = budget
        .reimbursables()
        .iter()
        .filter(|t| !t.reimbursed && t.amount <= 0)
        .cloned()
        .collect();
    s.call_on_id(
        OUTFLOWS_TABLE_ID,
        |v: &mut cursive::views::OnEventView<
            super::txn_table::TxnTableView,
        >| {
            let v = v.get_inner_mut();
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
            let idx =
                row.and_then(|id| outflows.iter().position(|t| t.id == id));
            v.set_items(outflows);
            if let Some(idx) = idx {
                v.set_selected_item(idx);
            }
        },
    )
    .unwrap();

    render_selected_total(s);
}

fn select(
    v: &mut cursive::views::LinearLayout,
) -> Option<cursive::event::EventResult> {
    let idx = v.get_focus_index();
    let child = v.get_child_mut(idx).unwrap();
    child.call_on_any(
        &cursive::view::Selector::Id(INFLOWS_TABLE_ID),
        Box::new(|v| {
            v.downcast_mut::<cursive::views::IdView<
                cursive::views::OnEventView<super::txn_table::TxnTableView>,
            >>()
            .map(|v| v.on_event(cursive::event::Event::Char(' ')));
        }),
    );
    child.call_on_any(
        &cursive::view::Selector::Id(OUTFLOWS_TABLE_ID),
        Box::new(|v| {
            v.downcast_mut::<cursive::views::IdView<
                cursive::views::OnEventView<super::txn_table::TxnTableView>,
            >>()
            .map(|v| v.on_event(cursive::event::Event::Char(' ')));
        }),
    );
    Some(cursive::event::EventResult::with_cb(|s| {
        render_selected_total(s);
    }))
}

fn render_selected_total(s: &mut cursive::Cursive) {
    let inflows: Vec<_> = s
        .call_on_id(
            INFLOWS_TABLE_ID,
            |v: &mut cursive::views::OnEventView<
                super::txn_table::TxnTableView,
            >| {
                v.get_inner_mut()
                    .borrow_items()
                    .iter()
                    .filter(|t| t.selected)
                    .map(|t| t.amount)
                    .collect()
            },
        )
        .unwrap();
    let outflows: Vec<_> = s
        .call_on_id(
            OUTFLOWS_TABLE_ID,
            |v: &mut cursive::views::OnEventView<
                super::txn_table::TxnTableView,
            >| {
                v.get_inner_mut()
                    .borrow_items()
                    .iter()
                    .filter(|t| t.selected)
                    .map(|t| t.amount)
                    .collect()
            },
        )
        .unwrap();
    let outflow: i64 = outflows.iter().sum();
    let inflow: i64 = inflows.iter().sum();
    let amount = outflow + inflow;
    s.call_on_id(SELECTED_TOTAL_ID, |v: &mut cursive::views::TextView| {
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
