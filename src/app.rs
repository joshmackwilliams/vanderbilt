use super::commands::command_from_str;
use super::ui::tui::TUI;
use super::ui::UI;
use crate::commands::AppCommand;
use crate::states::AppState;
use crate::states::SetupState;

pub fn run() {
    let ui: Box<dyn UI> = Box::new(TUI::new().expect("Failed to initialize UI"));
    let app = App::new(ui);
    app.run();
}

pub struct App {
    pub state: AppState,
    pub ui: Box<dyn UI>,
    pub undo: Vec<AppCommand>,
}

impl App {
    fn new(ui: Box<dyn UI>) -> Self {
        Self {
            state: SetupState::new().into(),
            ui,
            undo: Vec::new(),
        }
    }

    fn run(mut self) {
        self.ui.display_message("Welcome to Vanderbilt!");
        while !self.state.should_exit() {
            let command = command_from_str(&self.ui.get_input(&self.state));
            if let Result::Err(e) = command.execute(&mut self) {
                self.ui.display_error(&e);
            }
        }
    }
}
