use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Panel};

#[derive(Debug, PartialEq, Eq)]
pub enum HandleResult {
    Handled,
    Quit,
    NotHandled,
}

pub fn handle_key(app: &mut App, key: KeyEvent) -> HandleResult {
    use KeyCode::*;

    // Very important global shortcuts
    match key.code {
        Char('q') | Esc => return HandleResult::Quit,

        // Panel switching - always available
        Char('1') => {
            app.set_focus(Panel::Project);
            return HandleResult::Handled;
        }
        Char('2') => {
            app.set_focus(Panel::Dependencies);
            return HandleResult::Handled;
        }
        Char('4') => {
            app.set_focus(Panel::Details);
            return HandleResult::Handled;
        }

        _ => {}
    }

    // Focused panel specific handling
    match app.focused_panel {
        Panel::Dependencies => match key.code {
            Down | Char('j') => {
                app.next();
                HandleResult::Handled
            }
            Up | Char('k') => {
                app.previous();
                HandleResult::Handled
            }
            // Char('t') => toggle_tree_view(app),  // future
            // Char('u') => upgrade_selected(app),   // future
            _ => HandleResult::NotHandled,
        },

        // Other panels usually don't handle many keys yet
        _ => HandleResult::NotHandled,
    }
}
