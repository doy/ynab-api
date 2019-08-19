pub struct App {
    cursive: cursive::Cursive,
}

impl App {
    pub fn new(budget: crate::ynab::Budget) -> Self {
        let mut app = cursive::Cursive::default();
        let term_width = app.screen_size().x;
        app.set_theme(Self::theme());
        app.add_global_callback('q', |s| s.quit());

        let mut layout = cursive::views::LinearLayout::vertical();
        layout.add_child(cursive::views::TextView::new(format!(
            "Budget: {} ({})\n{}",
            budget.name(),
            budget.id(),
            "=".repeat(term_width),
        )));

        layout.add_child(crate::views::TxnTables::new("txn_tables", &budget));

        app.set_user_data(budget);
        app.add_fullscreen_layer(layout);

        Self { cursive: app }
    }

    pub fn run(&mut self) {
        self.cursive.run();
    }

    fn theme() -> cursive::theme::Theme {
        let mut palette = cursive::theme::Palette::default();
        palette[cursive::theme::PaletteColor::Background] =
            cursive::theme::Color::TerminalDefault;
        palette[cursive::theme::PaletteColor::View] =
            cursive::theme::Color::TerminalDefault;
        palette[cursive::theme::PaletteColor::Primary] =
            cursive::theme::Color::TerminalDefault;
        cursive::theme::Theme {
            shadow: false,
            borders: cursive::theme::BorderStyle::Simple,
            palette,
        }
    }
}
