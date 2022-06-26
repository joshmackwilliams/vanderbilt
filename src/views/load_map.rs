use crate::model::game::GameMap;

pub struct LoadMapView<'a> {
    map: &'a mut Option<GameMap>,
}

impl<'a> LoadMapView<'a> {
    pub fn new(map: &'a mut Option<GameMap>) -> Self {
        Self { map }
    }

    pub fn load(&mut self, map: GameMap) {
        *self.map = Option::Some(map);
    }
}
