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

    fn setup_simulated_transition(
        self: Box<Self>,
    ) -> Result<Box<dyn crate::transitions::SetupSimulatedTransition>, Box<dyn AppState>> {
        Result::Err(self)
    }
}
