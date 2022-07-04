use crate::model::game_map::GameMap;

pub struct LoadMapView<'a> {
    map: &'a mut Option<GameMap>,
}

impl<'a> LoadMapView<'a> {
    pub fn new(map: &'a mut Option<GameMap>) -> Self {
        Self { map }
    }

    pub fn load(&mut self, map: Option<GameMap>) -> Option<GameMap> {
        std::mem::replace(self.map, map)
    }
}
