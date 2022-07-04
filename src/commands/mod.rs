use crate::app::App;

mod exit;
mod invalid;
mod load_map;
mod undo;
mod undo_load_map;

pub use exit::ExitCommand;
pub use invalid::InvalidCommand;
pub use load_map::LoadMapCommand;
pub use undo::UndoCommand;
pub use undo_load_map::UndoLoadMapCommand;

pub trait AppCommand {
    fn execute(self: Box<Self>, app: &mut App) -> Result<(), String>;
}

pub fn command_from_str(command: &str) -> Box<dyn AppCommand> {
    let mut command = command.split(' ');
    match command.next() {
        Option::Some("exit") => Box::new(ExitCommand::new()),
        Option::Some("load_map") => Box::new(LoadMapCommand::new(command)),
        Option::Some("undo") => Box::new(UndoCommand::new()),
        _ => Box::new(InvalidCommand::new()),
    }
}
