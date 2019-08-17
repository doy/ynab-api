pub fn theme() -> cursive::theme::Theme {
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
