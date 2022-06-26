use super::AppCommand;
use crate::states::AppState;
use derive_more::Constructor;

#[derive(Constructor, Default)]
pub struct InvalidCommand {}

impl AppCommand for InvalidCommand {
    fn execute(
        self: Box<Self>,
        state: Box<dyn AppState>,
        ui: &mut dyn crate::ui::UI,
    ) -> Box<dyn AppState> {
        ui.display_message("Invalid command!");
        state
    }
}
