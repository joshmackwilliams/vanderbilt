use super::AppCommand;
use super::UndoLoadMapCommand;
use crate::app::App;
use crate::model::game_map::GameMap;
use std::fs::read_to_string;

pub struct LoadMapCommand {
    filename: Option<String>,
}

impl LoadMapCommand {
    pub fn new<'a, T: Iterator<Item = &'a str>>(mut args: T) -> Self {
        Self {
            filename: args.next().map(String::from),
        }
    }
}

impl AppCommand for LoadMapCommand {
    fn execute(self: Box<Self>, app: &mut App) -> Result<(), String> {
        let mut load_map_view = match app.state.load_map_view() {
            Some(v) => v,
            None => {
                return Result::Err("This command is not available".to_owned());
            }
        };
        let filename = self
            .filename
            .ok_or_else(|| "Please provide a filename!".to_owned())?;
        let map = read_to_string(&filename)
            .map_err(|e| format!("Error reading from {}: {}", filename, e))?;
        let map =
            serde_json::de::from_str(&map).map_err(|e| format!("Error parsing map file: {}", e))?;
        let map = GameMap::new(map).map_err(|e| format!("Error loading game: {:?}", e))?;
        app.ui.display_message(&format!(
            "Loaded {} colors, {} cities, {} routes, and {} destinations",
            map.colors.colors().len(),
            map.cities.cities().len(),
            map.routes.len(),
            map.destinations.len()
        ));
        let map = load_map_view.load(Option::Some(map));
        app.undo.push(Box::new(UndoLoadMapCommand::new(map)));
        Result::Ok(())
    }
}
