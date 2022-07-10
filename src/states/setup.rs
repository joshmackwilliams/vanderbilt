use crate::{
    model::game_map::GameMap,
    views::{AddPlayerView, LoadMapView, VisualizeView},
};

use super::{AppState, AppStateTrait};

pub struct SetupState {
    map: Option<GameMap>,
}

impl SetupState {
    pub fn new() -> Self {
        Self { map: Option::None }
    }
}

impl Default for SetupState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppStateTrait for SetupState {
    fn load_map_view(&mut self) -> Option<LoadMapView<'_>> {
        Option::Some(LoadMapView::new(&mut self.map))
    }

    fn visualize_view(&self) -> Option<VisualizeView<'_>> {
        self.map.as_ref().map(VisualizeView::new)
    }

    fn add_player_view(&mut self) -> Option<AddPlayerView<'_>> {
        Option::None
    }
}

impl From<SetupState> for AppState {
    fn from(state: SetupState) -> Self {
        Self::SetupState(state)
    }
}
