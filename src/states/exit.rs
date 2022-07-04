use super::AppState;

#[derive(Default)]
pub struct ExitState {}

impl ExitState {
    pub fn new() -> Self {
        Self {}
    }
}

impl AppState for ExitState {
    fn should_exit(&self) -> bool {
        true
    }
}
