use super::{
    city::{Cities, City, CityName},
    game_map::GameMapCreationError,
};
use crate::{errors::CityNotFoundError, model::id::Id};
use derive_more::From;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct DestinationDTO {
    pub from: CityName,
    pub to: CityName,
    pub points: usize,
}

#[derive(Error, Debug, From)]
pub enum DestinationCreationError {
    #[error("City not found: {0}")]
    CityNotFound(CityNotFoundError),
}

impl From<DestinationCreationError> for GameMapCreationError {
    fn from(e: DestinationCreationError) -> Self {
        match e {
            DestinationCreationError::CityNotFound(e) => Self::CityNotFound(e),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Destination {
    from: Id<City>,
    to: Id<City>,
    points: usize,
}

impl Destination {
    pub fn new(dto: DestinationDTO, cities: &Cities) -> Result<Self, DestinationCreationError> {
        let DestinationDTO { from, to, points } = dto;
        let from = cities
            .by_name(&from)
            .ok_or_else(|| CityNotFoundError::from(from))?;
        let to = cities
            .by_name(&to)
            .ok_or_else(|| CityNotFoundError::from(to))?;
        Result::Ok(Self { from, to, points })
    }
}
