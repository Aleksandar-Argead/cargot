use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::{App, Panel};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.is_focused(Panel::Project);

    let border_style = if is_focused {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let title_style = if is_focused {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let text = "... project info ...";

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .title(Span::styled(" Project ", title_style))
            .borders(Borders::ALL)
            .border_style(border_style)
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(paragraph, area);
}
