use super::AppCommand;
use crate::{app::App, states::ExitState};

pub struct ExitCommand {}

impl ExitCommand {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(self, app: &mut App) -> Result<(), String> {
        app.ui.display_message("Exiting...");
        app.state = ExitState::new().into();
        Result::Ok(())
    }
}

impl Default for ExitCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ExitCommand> for AppCommand {
    fn from(state: ExitCommand) -> Self {
        Self::ExitCommand(state)
    }
}
