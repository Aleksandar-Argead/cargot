use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::app::{App, Panel};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.is_focused(Panel::Dependencies);

    let border_style = if is_focused {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
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
                .border_style(border_style)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, area, &mut app.deps_state.clone());
}
