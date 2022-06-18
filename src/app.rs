use super::commands::command_from_str;
use super::model::game_builder::GameBuilder;
use super::ui::cli::CLI;
use super::ui::UI;

pub enum AppState {
    GameNotStarted(GameBuilder),
    Exited,
}

pub fn run() {
    let mut ui: Box<dyn UI> = Box::new(CLI::new());
    let mut state = AppState::GameNotStarted(GameBuilder::new());
    ui.display_message("Welcome to Vanderbilt!");
    loop {
        let command = command_from_str(&ui.get_input(&state));
        command.execute(&mut state, ui.as_mut());
        if let AppState::Exited = state {
            break;
        }
    }
}
