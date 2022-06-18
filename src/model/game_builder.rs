use super::common::game::GameCommon;

pub struct GameBuilder {
    pub common: Option<GameCommon>,
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
