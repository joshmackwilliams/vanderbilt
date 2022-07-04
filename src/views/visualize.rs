use crate::model::game_map::GameMap;

pub struct VisualizeView<'a> {
    map: &'a GameMap,
}

impl<'a> VisualizeView<'a> {
    pub fn new(map: &'a GameMap) -> Self {
        Self { map }
    }

    pub fn map(&self) -> &'a GameMap {
        self.map
    }
}
