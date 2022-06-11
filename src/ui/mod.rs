pub mod cli;

pub trait UI {
    fn display_message(&self, message: &str);
}
