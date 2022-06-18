use crate::app::AppState;
use crate::ui::UI;

use super::GameCommand;

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
        let filename = match self.filename.take() {
            Option::Some(filename) => filename,
            Option::None => {
                ui.display_message("Please provide a filename!");
                return;
            }
        };
    }
}
