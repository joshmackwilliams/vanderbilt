use crate::{
    model::game_map::GameMap,
    views::{LoadMapView, VisualizeView},
};

use super::AppState;

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

impl AppState for SetupState {
    fn load_map_view(&mut self) -> Option<LoadMapView<'_>> {
        Option::Some(LoadMapView::new(&mut self.map))
    }

    fn visualize_view(&self) -> Option<VisualizeView<'_>> {
        self.map.as_ref().map(VisualizeView::new)
    }
}
