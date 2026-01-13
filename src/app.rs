use crate::{cargo_ops::metadata::load_dependencies, models::dependency::Dependency};
use color_eyre::eyre::Result;
use ratatui::widgets::ListState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Project = 1,
    Dependencies = 2,
    // Scripts = 3,     // later
    Details = 4,
}

#[derive(Debug)]
pub struct App {
    pub focused_panel: Panel,

    pub dependencies: Vec<Dependency>,
    pub deps_state: ListState,
    // pub popup: Option<String>,
}

impl App {
    pub fn new() -> Result<Self> {
        let dependencies = load_dependencies().unwrap_or_default();

        let mut deps_state = ListState::default();

        if !dependencies.is_empty() {
            deps_state.select(Some(0));
        }

        Ok(Self {
            focused_panel: Panel::Dependencies,
            dependencies,
            deps_state,
        })
    }

    pub fn selected_dep(&self) -> Option<&Dependency> {
        self.deps_state
            .selected()
            .and_then(|i| self.dependencies.get(i))
    }

    pub fn set_focus(&mut self, panel: Panel) {
        self.focused_panel = panel;
    }

    pub fn is_focused(&self, panel: Panel) -> bool {
        self.focused_panel == panel
    }

    pub fn next(&mut self) {
        let i = self.deps_state.selected().unwrap_or(0);
        let next = (i + 1) % self.dependencies.len();
        self.deps_state.select(Some(next));
    }

    pub fn previous(&mut self) {
        let i = self.deps_state.selected().unwrap_or(0);
        let prev = if i == 0 {
            self.dependencies.len().saturating_sub(1)
        } else {
            i - 1
        };
        self.deps_state.select(Some(prev));
    }
}
