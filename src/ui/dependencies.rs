use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::{App, Panel};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let focus_style = if app.is_focused(Panel::Dependencies) {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Gray)
    };

    let items: Vec<ListItem> = app
        .dependencies
        .iter()
        .map(|dep| ListItem::new(dep.compact_line()))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Dependencies ")
                .borders(Borders::ALL)
                .border_style(focus_style),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, area, &mut app.deps_state.clone());
}
