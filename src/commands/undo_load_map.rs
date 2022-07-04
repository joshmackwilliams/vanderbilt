use crate::model::game_map::GameMap;

use super::AppCommand;
use derive_more::Constructor;

#[derive(Constructor)]
pub struct UndoLoadMapCommand {
    map: Option<GameMap>,
}

impl AppCommand for UndoLoadMapCommand {
    fn execute(self: Box<Self>, app: &mut crate::app::App) -> Result<(), String> {
        app.state
            .load_map_view()
            .ok_or_else(|| "No load map view in this state".to_owned())?
            .load(self.map);
        Result::Ok(())
    }
}
