use super::AppCommand;
use crate::model::game::{GameMap, GameMapDTO};
use crate::states::AppState;
use crate::ui::UI;
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
    fn execute(
        mut self: Box<Self>,
        mut state: Box<dyn AppState>,
        ui: &mut dyn UI,
    ) -> Box<dyn AppState> {
        let mut load_map_view = match state.load_map_view() {
            Some(v) => v,
            None => {
                ui.display_error("This command is not available");
                return state;
            }
        };
        let filename = match self.filename.take() {
            Option::Some(filename) => filename,
            Option::None => {
                ui.display_message("Please provide a filename!");
                return state;
            }
        };
        let map: String = match read_to_string(&filename) {
            Result::Ok(x) => x,
            Result::Err(e) => {
                ui.display_message(&format!("Error reading from {}: {}", filename, e));
                return state;
            }
        };
        let map: GameMapDTO = match serde_json::de::from_str(&map) {
            Result::Ok(x) => x,
            Result::Err(e) => {
                ui.display_message(&format!("Error parsing map file: {}", e));
                return state;
            }
        };
        let map = match GameMap::new(map) {
            Result::Ok(x) => x,
            Result::Err(e) => {
                ui.display_message(&format!("Error loading game: {:?}", e));
                return state;
            }
        };
        ui.display_message(&format!(
            "Loaded {} colors, {} cities, {} routes, and {} destinations",
            map.colors.colors().len(),
            map.cities.cities().len(),
            map.routes.len(),
            map.destinations.len()
        ));
        load_map_view.load(map);
        state
    }
}
