use super::UI;
use crate::app::AppState;
use std::io::{BufRead, Write};

pub struct CLI {}

impl CLI {
    pub fn new() -> Self {
        CLI {}
    }
}

impl UI for CLI {
    fn display_message(&mut self, message: &str) {
        println!("{}", message);
    }

    fn get_input(&mut self, _app_state: &AppState) -> String {
        print!("> ");
        std::io::stdout().lock().flush().unwrap();
        std::io::stdin().lock().lines().next().unwrap().unwrap()
    }
}

impl Default for CLI {
    fn default() -> Self {
        Self::new()
    }
}
