mod checks;
mod display;
mod paths;
mod views;
mod ynab;

use std::io::Read;

fn main() {
    let mut key = String::new();
    std::fs::File::open(paths::api_key())
        .unwrap()
        .read_to_string(&mut key)
        .unwrap();
    let client = ynab::Client::new(&key.trim());
    let budget = client.into_default_budget();

    checks::run_checks(&budget);

    let mut app = cursive::Cursive::default();
    let term_width = app.screen_size().x;
    app.set_theme(display::theme());
    app.add_global_callback('q', |s| s.quit());

    let mut layout = cursive::views::LinearLayout::vertical();
    layout.add_child(cursive::views::TextView::new(format!(
        "Budget: {} ({})\n{}",
        budget.name(),
        budget.id(),
        "=".repeat(term_width),
    )));

    layout.add_child(views::txn_tables(&budget));

    app.set_user_data(budget);
    app.add_fullscreen_layer(layout);
    app.run();
}
