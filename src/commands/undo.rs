use super::AppCommand;
use crate::app::App;
use derive_more::Constructor;

#[derive(Constructor)]
pub struct UndoCommand {}

impl UndoCommand {
    pub fn execute(self, app: &mut App) -> Result<(), String> {
        app.undo
            .pop()
            .ok_or_else(|| "Undo history is empty!".to_owned())?
            .execute(app)
            .map_err(|e| format!("Error undoing command: {}", e))?;
        app.ui.display_message("Undo");
        Result::Ok(())
    }
}

impl From<UndoCommand> for AppCommand {
    fn from(state: UndoCommand) -> Self {
        Self::UndoCommand(state)
    }
}
