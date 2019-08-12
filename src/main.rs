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

    layout.add_child(views::txn_tables(&budget));

    app.add_fullscreen_layer(layout);
    app.run();
}
