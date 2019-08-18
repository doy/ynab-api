use cursive::view::Identifiable;

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

        TxnTables {
            view: cursive::views::OnEventView::new(layout.with_id(id)),
        }
    }
}

fn inflows_table(budget: &crate::ynab::Budget) -> super::txn_table::TxnTable {
    let inflows = budget
        .reimbursables()
        .iter()
        .filter(|t| !t.reimbursed && t.amount > 0)
        .cloned()
        .collect();
    super::txn_table::TxnTable::new(inflows, "inflows_table")
}

fn outflows_table(
    budget: &crate::ynab::Budget,
) -> super::txn_table::TxnTable {
    let outflows = budget
        .reimbursables()
        .iter()
        .filter(|t| !t.reimbursed && t.amount <= 0)
        .cloned()
        .collect();
    super::txn_table::TxnTable::new(outflows, "outflows_table")
}
