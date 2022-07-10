use super::{AppState, AppStateTrait};

#[derive(Default)]
pub struct ExitState {}

impl ExitState {
    pub fn new() -> Self {
        Self {}
    }
}

impl AppStateTrait for ExitState {
    fn should_exit(&self) -> bool {
        true
    }
}

impl From<ExitState> for AppState {
    fn from(state: ExitState) -> Self {
        Self::ExitState(state)
    }
}
