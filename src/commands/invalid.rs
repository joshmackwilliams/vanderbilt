use super::AppCommand;
use crate::app::App;
use derive_more::Constructor;

#[derive(Constructor, Default)]
pub struct InvalidCommand {}

impl InvalidCommand {
    pub fn execute(self, _app: &mut App) -> Result<(), String> {
        Result::Err("Invalid command".to_owned())
    }
}

impl From<InvalidCommand> for AppCommand {
    fn from(state: InvalidCommand) -> Self {
        Self::InvalidCommand(state)
    }
}
