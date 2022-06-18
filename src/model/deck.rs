use super::color::{Color, ColorDB};
use id_arena::Id;
use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct DeckDTO {
    pub distribution: std::collections::HashMap<String, usize>,
}

pub enum DeckCreationError {
    ColorNotFoundError(String),
}

#[derive(Clone)]
pub struct DeckDistribution {
    cards: HashMap<Id<Color>, usize>,
}

impl DeckDistribution {
    pub fn new(dto: DeckDTO, colors: &ColorDB) -> Result<Self, DeckCreationError> {
        let distribution: Result<HashMap<Id<Color>, usize>, String> = dto
            .distribution
            .into_iter()
            .map(|(name, count)| match colors.by_name.get(&name) {
                Option::Some(id) => Result::Ok((*id, count)),
                Option::None => Result::Err(name),
            })
            .collect();
        distribution
            .map(|distribution| Self {
                cards: distribution,
            })
            .map_err(DeckCreationError::ColorNotFoundError)
    }
}

pub struct PerfectInformationDeck {
    initial_distribution: DeckDistribution,
    current_distribution: DeckDistribution,
    random_distribution: WeightedIndex<usize>,
    color_mapping: Vec<Id<Color>>,
}

impl PerfectInformationDeck {
    pub fn new(initial_distribution: DeckDistribution) -> Self {
        let (color_mapping, weights): (Vec<Id<Color>>, Vec<usize>) =
            initial_distribution.cards.iter().unzip();
        let random_distribution =
            WeightedIndex::new(weights).expect("Tried to create an empty deck");
        let current_distribution = initial_distribution.clone();
        Self {
            initial_distribution,
            current_distribution,
            random_distribution,
            color_mapping,
        }
    }

    pub fn shuffle(&mut self) {
        self.current_distribution = self.initial_distribution.clone();
        let (color_mapping, weights): (Vec<Id<Color>>, Vec<usize>) =
            self.initial_distribution.cards.iter().unzip();
        let random_distribution =
            WeightedIndex::new(weights).expect("Tried to create an empty deck");
        self.color_mapping = color_mapping;
        self.random_distribution = random_distribution;
    }

    pub fn draw(&mut self) -> Id<Color> {
        let index = self.random_distribution.sample(&mut thread_rng());
        let color = self.color_mapping[index];
        let new_weight = self.current_distribution.cards[&color] - 1;
        self.current_distribution.cards.insert(color, new_weight);
        if self
            .random_distribution
            .update_weights(&[(index, &new_weight)])
            .is_err()
        {
            self.shuffle();
        }
        color
    }
}
