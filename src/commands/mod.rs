use super::app::AppState;
use crate::ui::UI;

pub mod exit;
pub mod invalid;
pub mod load_map;

pub use exit::ExitCommand;
pub use invalid::InvalidCommand;
pub use load_map::LoadMapCommand;

pub trait GameCommand {
    fn execute(self: Box<Self>, app_state: &mut AppState, ui: &mut dyn UI);
}

pub fn command_from_str(command: &str) -> Box<dyn GameCommand> {
    let mut command = command.split(' ');
    match command.next() {
        Option::Some("exit") => Box::new(ExitCommand::new()),
        Option::Some("load_map") => Box::new(LoadMapCommand::new(command)),
        _ => Box::new(InvalidCommand::new()),
    }
}
