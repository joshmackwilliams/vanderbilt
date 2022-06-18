use super::{city::CityId, game::GameCreationError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DestinationId(pub u16);

#[derive(Serialize, Deserialize)]
pub struct DestinationDTO {
    pub from: String,
    pub to: String,
    pub points: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Destination {
    pub from: CityId,
    pub to: CityId,
    pub points: usize,
}

impl Destination {
    pub fn new(
        dto: DestinationDTO,
        cities_by_name: &HashMap<String, CityId>,
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
