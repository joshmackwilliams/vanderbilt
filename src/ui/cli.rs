use super::UI;

pub struct CLI {}

impl CLI {
    pub fn new() -> Self {
        CLI {}
    }
}

impl UI for CLI {
    fn display_message(&self, message: &str) {
        println!("{}", message);
    }
}

impl Default for CLI {
    fn default() -> Self {
        Self::new()
    }
}
