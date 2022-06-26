mod exit;
mod generic_setup;

pub use exit::ExitState;
pub use generic_setup::GenericSetupState;

use crate::{
    transitions::SetupSimulatedTransition,
    views::{LoadMapView, VisualizeView},
};

pub trait AppState {
    fn load_map_view(&mut self) -> Option<LoadMapView<'_>> {
        Option::None
    }

    fn visualize_view(&self) -> Option<VisualizeView<'_>> {
        Option::None
    }

    fn setup_simulated_transition(
        self: Box<Self>,
    ) -> Result<Box<dyn SetupSimulatedTransition>, Box<dyn AppState>>;

    fn should_exit(&self) -> bool {
        false
    }
}
