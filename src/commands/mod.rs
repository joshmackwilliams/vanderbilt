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

pub enum AppCommand {
    ExitCommand(ExitCommand),
    InvalidCommand(InvalidCommand),
    LoadMapCommand(LoadMapCommand),
    UndoCommand(UndoCommand),
    UndoLoadMapCommand(UndoLoadMapCommand),
}

impl AppCommand {
    pub fn execute(self, app: &mut App) -> Result<(), String> {
        match self {
            Self::ExitCommand(x) => x.execute(app),
            Self::InvalidCommand(x) => x.execute(app),
            Self::LoadMapCommand(x) => x.execute(app),
            Self::UndoCommand(x) => x.execute(app),
            Self::UndoLoadMapCommand(x) => x.execute(app),
        }
    }
}

pub fn command_from_str(command: &str) -> AppCommand {
    let mut command = command.split(' ');
    match command.next() {
        Option::Some("exit") => ExitCommand::new().into(),
        Option::Some("load_map") => LoadMapCommand::new(command).into(),
        Option::Some("undo") => UndoCommand::new().into(),
        _ => InvalidCommand::new().into(),
    }
}
