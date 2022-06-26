use super::game::GameMap;

pub struct GameBuilder {
    pub common: Option<GameMap>,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self {
            common: Option::None,
        }
    }
}

impl Default for GameBuilder {
    fn default() -> Self {
        Self::new()
    }
}
