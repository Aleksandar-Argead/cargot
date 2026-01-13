use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, Panel};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let focus_style = if app.is_focused(Panel::Project) {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Gray)
    };

    let text = format!(
        "Project: my-awesome-crate\n\
         Version: 0.1.0\n\
         Toolchain: stable (rustc 1.81.0)\n\
         Outdated: 5    Vulns: 1"
    );

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .title(" Project ")
            .borders(Borders::ALL)
            .border_style(focus_style),
    );

    frame.render_widget(paragraph, area);
}
