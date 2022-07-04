use super::AppCommand;
use crate::{app::App, states::ExitState};

pub struct ExitCommand {}

impl ExitCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl AppCommand for ExitCommand {
    fn execute(self: Box<Self>, app: &mut App) -> Result<(), String> {
        app.ui.display_message("Exiting...");
        app.state = Box::new(ExitState::new());
        Result::Ok(())
    }
}

impl Default for ExitCommand {
    fn default() -> Self {
        Self::new()
    }
}
