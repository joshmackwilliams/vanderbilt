use super::AppCommand;
use crate::app::App;
use derive_more::Constructor;

#[derive(Constructor)]
pub struct UndoCommand {}

impl AppCommand for UndoCommand {
    fn execute(self: Box<Self>, app: &mut App) -> Result<(), String> {
        app.undo
            .pop()
            .ok_or_else(|| "Undo history is empty!".to_owned())?
            .execute(app)
            .map_err(|e| format!("Error undoing command: {}", e))
    }
}
