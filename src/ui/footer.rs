use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, Panel};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let text = match app.focused_panel {
        Panel::Project => "1:Project  2:Deps  q:quit",
        Panel::Dependencies => "↑↓jk: navigate  t:tree/flat  u:upgrade  q:quit",
        Panel::Details => "↑↓: scroll   q:quit",
    };

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP));

    frame.render_widget(paragraph, area);
}
