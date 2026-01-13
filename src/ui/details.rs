use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, Panel};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let focus_style = if app.is_focused(Panel::Details) {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Gray)
    };

    let text = match app.selected_dep() {
        Some(dep) => format!("Selected:\n{} {}", dep.name, dep.version_req),
        None => "No dependency selected".to_string(),
    };

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .title(" Details ")
            .borders(Borders::ALL)
            .border_style(focus_style),
    );

    frame.render_widget(paragraph, area);
}
