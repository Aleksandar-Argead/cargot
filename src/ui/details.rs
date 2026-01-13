use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::{App, Panel};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.is_focused(Panel::Details);

    let border_style = if is_focused {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let text = match app.selected_dep() {
        Some(dep) => format!("Selected:\n{} {}", dep.name, dep.version_req),
        None => "No dependency selected".to_string(),
    };

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .title(" Details ")
            .borders(Borders::ALL)
            .border_style(border_style)
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(paragraph, area);
}
