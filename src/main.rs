mod checks;
mod display;
mod views;
mod ynab;

fn main() {
    let key = std::env::args().nth(1).unwrap();
    let client = ynab::Client::new(&key);
    let budget = client.default_budget();

    checks::run_checks(&budget);

    let mut app = cursive::Cursive::default();
    app.set_theme(display::theme());
    app.add_global_callback('q', |s| s.quit());

    let mut layout = cursive::views::LinearLayout::vertical();
    layout.add_child(cursive::views::TextView::new(format!(
        "Budget: {} ({})",
        budget.name(),
        budget.id()
    )));

    let inflows_table = views::inflows_table(&budget);
    layout.add_child(views::vi_view(
        cursive::views::CircularFocus::wrap_arrows(
            cursive::views::BoxView::with_min_height(
                std::cmp::min(std::cmp::max(inflows_table.len(), 1), 5) + 2,
                cursive::views::BoxView::with_full_width(inflows_table),
            ),
        ),
    ));

    let outflows_table = views::outflows_table(&budget);
    layout.add_child(views::vi_view(
        cursive::views::CircularFocus::wrap_arrows(
            cursive::views::BoxView::with_full_screen(outflows_table),
        ),
    ));

    app.add_fullscreen_layer(layout);
    app.run();
}
