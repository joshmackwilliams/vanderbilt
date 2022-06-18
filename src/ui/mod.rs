use crate::app::AppState;

pub mod cli;
pub mod tui;

pub trait UI {
    fn display_message(&mut self, message: &str);
    fn get_input(&mut self, app_state: &AppState) -> String;
}
