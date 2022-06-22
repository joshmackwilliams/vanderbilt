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
        let from = cities_by_name
            .get(&from)
            .map_or_else(
                || Result::Err(GameCreationError::CityNotFoundError(from)),
                Result::Ok,
            )
            .copied()?;
        let to = cities_by_name
            .get(&to)
            .map_or_else(
                || Result::Err(GameCreationError::CityNotFoundError(to)),
                Result::Ok,
            )
            .copied()?;
        Result::Ok(Self { from, to, points })
    }
}
