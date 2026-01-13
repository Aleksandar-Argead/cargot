use color_eyre::eyre::Result;
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use std::io::{self, stdout};

mod app;
mod cargo_ops;
mod models;
mod ui;

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;

    // Terminal setup
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;

    let app_result = run_app(&mut terminal, &mut app);

    // Cleanup
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    // Show terminal cursor again
    println!();

    app_result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| ui::layout::draw(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    // More keys will be handled later via app / handler
                    _ => {}
                }
            }
        }
    }
}
