use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::app::App;
use crate::ui::{dependencies, details, footer, project};

pub fn draw(frame: &mut Frame, app: &App) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // main content
            Constraint::Length(3), // footer
        ])
        .split(frame.area());

    let main_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
        .split(vertical[0]);

    let left_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9), // project status
            Constraint::Min(0),    // dependencies list
        ])
        .split(main_horizontal[0]);

    // Render components
    project::draw(frame, app, left_vertical[0]);
    dependencies::draw(frame, app, left_vertical[1]);
    details::draw(frame, app, main_horizontal[1]);
    footer::draw(frame, app, vertical[1]);
}
