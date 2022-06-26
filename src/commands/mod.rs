use crate::{states::AppState, ui::UI};

mod exit;
mod invalid;
mod load_map;

pub use exit::ExitCommand;
pub use invalid::InvalidCommand;
pub use load_map::LoadMapCommand;

pub trait AppCommand {
    fn execute(self: Box<Self>, app_state: Box<dyn AppState>, ui: &mut dyn UI)
        -> Box<dyn AppState>;
}

pub fn command_from_str(command: &str) -> Box<dyn AppCommand> {
    let mut command = command.split(' ');
    match command.next() {
        Option::Some("exit") => Box::new(ExitCommand::new()),
        Option::Some("load_map") => Box::new(LoadMapCommand::new(command)),
        _ => Box::new(InvalidCommand::new()),
    }
}
