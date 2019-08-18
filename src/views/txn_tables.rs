use cursive::view::Identifiable;

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

        TxnTables {
            view: cursive::views::OnEventView::new(layout.with_id(id)),
        }
    }
}
