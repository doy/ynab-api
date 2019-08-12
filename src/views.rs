mod txn_table;
pub use txn_table::txn_tables;

pub fn vi_view<V: cursive::view::View>(v: V) -> impl cursive::view::View {
    cursive::views::OnEventView::new(v)
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
}
