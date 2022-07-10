mod exit;
mod setup;

pub use exit::ExitState;
pub use setup::SetupState;
use trait_enum::trait_enum;

use crate::views::{AddPlayerView, LoadMapView, VisualizeView};

pub trait AppStateTrait {
    fn load_map_view(&mut self) -> Option<LoadMapView<'_>> {
        Option::None
    }

    fn visualize_view(&self) -> Option<VisualizeView<'_>> {
        Option::None
    }

    fn should_exit(&self) -> bool {
        false
    }

    fn add_player_view(&mut self) -> Option<AddPlayerView<'_>> {
        Option::None
    }
}

trait_enum! {
    pub enum AppState: AppStateTrait {
        ExitState,
        SetupState,
    }
}
