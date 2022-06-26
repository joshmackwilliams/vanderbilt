use crate::{
    model::game::GameMap,
    transitions::SetupSimulatedTransition,
    views::{LoadMapView, VisualizeView},
};

use super::AppState;

pub struct GenericSetupState {
    map: Option<GameMap>,
}

impl GenericSetupState {
    pub fn new() -> Self {
        Self { map: Option::None }
    }
}

impl Default for GenericSetupState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState for GenericSetupState {
    fn load_map_view(&mut self) -> Option<LoadMapView<'_>> {
        Option::Some(LoadMapView::new(&mut self.map))
    }

    fn visualize_view(&self) -> Option<VisualizeView<'_>> {
        self.map.as_ref().map(VisualizeView::new)
    }

    fn setup_simulated_transition(
        self: Box<Self>,
    ) -> Result<Box<dyn SetupSimulatedTransition>, Box<dyn AppState>> {
        Result::Ok(self)
    }
}

impl SetupSimulatedTransition for GenericSetupState {
    fn transition(self: Box<Self>) -> Box<dyn AppState> {
        todo!("Not yet implemented!")
    }
}
