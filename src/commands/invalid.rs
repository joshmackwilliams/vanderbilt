pub use super::GameCommand;

pub struct InvalidCommand {}

impl InvalidCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameCommand for InvalidCommand {
    fn execute(self: Box<Self>, _app_state: &mut crate::app::AppState, ui: &mut dyn crate::ui::UI) {
        ui.display_message("Invalid command!");
    }
}

impl Default for InvalidCommand {
    fn default() -> Self {
        Self::new()
    }
}
