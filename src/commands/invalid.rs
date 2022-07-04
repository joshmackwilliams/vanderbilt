use super::AppCommand;
use crate::app::App;
use derive_more::Constructor;

#[derive(Constructor, Default)]
pub struct InvalidCommand {}

impl AppCommand for InvalidCommand {
    fn execute(self: Box<Self>, _app: &mut App) -> Result<(), String> {
        Result::Err("Invalid command".to_owned())
    }
}
