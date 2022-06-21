use super::{
    city::{City, CityName},
    game::GameCreationError,
};
use crate::model::id::Id;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct DestinationDTO {
    pub from: CityName,
    pub to: CityName,
    pub points: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Destination {
    pub from: Id<City>,
    pub to: Id<City>,
    pub points: usize,
}

impl Destination {
    pub fn new(
        dto: DestinationDTO,
        cities_by_name: &HashMap<CityName, Id<City>>,
    ) -> Result<Self, GameCreationError> {
        let DestinationDTO { from, to, points } = dto;
        let from = *match cities_by_name.get(&from) {
            Option::Some(x) => Result::Ok(x),
            Option::None => Result::Err(GameCreationError::CityNotFound(from)),
        }?;
        let to = *match cities_by_name.get(&to) {
            Option::Some(x) => Result::Ok(x),
            Option::None => Result::Err(GameCreationError::CityNotFound(to)),
        }?;
        Result::Ok(Self { from, to, points })
    }
}
