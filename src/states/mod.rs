mod exit;
mod setup;

pub use exit::ExitState;
pub use setup::SetupState;

use crate::views::{LoadMapView, VisualizeView};

pub trait AppState {
    fn load_map_view(&mut self) -> Option<LoadMapView<'_>> {
        Option::None
    }

    fn visualize_view(&self) -> Option<VisualizeView<'_>> {
        Option::None
    }

    fn should_exit(&self) -> bool {
        false
    }
}
