use super::GameCommand;
use crate::app::AppState;
use crate::ui::UI;

pub struct ExitCommand {}

impl ExitCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameCommand for ExitCommand {
    fn execute(self: Box<Self>, state: &mut AppState, ui: &mut dyn UI) {
        *state = AppState::Exited;
        ui.display_message("Exiting...");
    }
}

impl Default for ExitCommand {
    fn default() -> Self {
        Self::new()
    }
}
