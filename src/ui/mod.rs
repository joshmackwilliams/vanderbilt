use crate::states::AppState;

pub mod cli;
pub mod tui;

pub trait UI {
    fn display_message(&mut self, message: &str);
    fn display_error(&mut self, error: &str);
    fn get_input(&mut self, state: &AppState) -> String;
}
