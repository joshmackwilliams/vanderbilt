use crate::states::AppState;

pub trait SetupSimulatedTransition {
    fn transition(self: Box<Self>) -> Box<dyn AppState>;
}
