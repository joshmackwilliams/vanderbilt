use super::AppCommand;
use crate::states::AppState;
use crate::states::ExitState;
use crate::ui::UI;

pub struct ExitCommand {}

impl ExitCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl AppCommand for ExitCommand {
    fn execute(self: Box<Self>, _state: Box<dyn AppState>, ui: &mut dyn UI) -> Box<dyn AppState> {
        ui.display_message("Exiting...");
        Box::new(ExitState::new())
    }
}

impl Default for ExitCommand {
    fn default() -> Self {
        Self::new()
    }
}
