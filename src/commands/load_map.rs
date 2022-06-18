use super::GameCommand;
use crate::app::AppState;
use crate::model::common::game::{GameCommon, GameDTO};
use crate::model::game_builder::GameBuilder;
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

impl GameCommand for LoadMapCommand {
    fn execute(mut self: Box<Self>, app_state: &mut AppState, ui: &mut dyn UI) {
        let builder: &mut GameBuilder = match app_state {
            AppState::GameNotStarted(builder) => builder,
            _ => {
                ui.display_message("This command is not available right now");
                return;
            }
        };
        let filename = match self.filename.take() {
            Option::Some(filename) => filename,
            Option::None => {
                ui.display_message("Please provide a filename!");
                return;
            }
        };
        let game_state: String = match read_to_string(&filename) {
            Result::Ok(x) => x,
            Result::Err(e) => {
                ui.display_message(&format!("Error reading from {}: {}", filename, e));
                return;
            }
        };
        let game_state: GameDTO = match serde_json::de::from_str(&game_state) {
            Result::Ok(x) => x,
            Result::Err(e) => {
                ui.display_message(&format!("Error parsing map file: {}", e));
                return;
            }
        };
        let game_state = match GameCommon::new(game_state) {
            Result::Ok(x) => x,
            Result::Err(e) => {
                ui.display_message(&format!("Error loading game: {}", e));
                return;
            }
        };
        ui.display_message(&format!(
            "Loaded {} colors, {} cities, {} routes, and {} destinations",
            game_state.colors.len(),
            game_state.cities.len(),
            game_state.routes.len(),
            game_state.destinations.len()
        ));
        builder.common = Option::Some(game_state);
    }
}
