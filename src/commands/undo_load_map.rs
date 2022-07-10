use crate::{app::App, model::game_map::GameMap};

use super::AppCommand;
use derive_more::Constructor;

#[derive(Constructor)]
pub struct UndoLoadMapCommand {
    map: Option<GameMap>,
}

impl UndoLoadMapCommand {
    pub fn execute(self, app: &mut App) -> Result<(), String> {
        app.state
            .load_map_view()
            .ok_or_else(|| "No load map view in this state".to_owned())?
            .load(self.map);
        Result::Ok(())
    }
}

impl From<UndoLoadMapCommand> for AppCommand {
    fn from(state: UndoLoadMapCommand) -> Self {
        Self::UndoLoadMapCommand(state)
    }
}
