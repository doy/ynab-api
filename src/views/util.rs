pub type FullView<T> = cursive::views::OnEventView<cursive::views::IdView<T>>;

pub fn dialog(s: &str) -> impl cursive::view::View {
    cursive::views::Panel::new(cursive::views::Dialog::info(s))
}
