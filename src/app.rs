use super::commands::command_from_str;
use super::ui::tui::TUI;
use super::ui::UI;
use crate::states::AppState;
use crate::states::GenericSetupState;

pub fn run() {
    let mut ui: Box<dyn UI> = Box::new(TUI::new().expect("Failed to initialize UI"));
    let mut state: Box<dyn AppState> = Box::new(GenericSetupState::default());
    ui.display_message("Welcome to Vanderbilt!");
    while !state.should_exit() {
        let command = command_from_str(&ui.get_input(state.as_ref()));
        state = command.execute(state, ui.as_mut());
    }
}
