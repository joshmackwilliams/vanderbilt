use super::city::Cities;
use super::color::Colors;
use super::destination::{Destination, DestinationCreationError, DestinationDTO};
use super::route::{Route, RouteCreationError, RouteDTO};
use crate::errors::{CityNotFoundError, ColorNotFoundError};
use crate::model::id::Id;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use typed_index_collections::TiVec;

#[derive(Serialize, Deserialize)]
pub struct GameMapDTO {
    cities: Cities,
    colors: Colors,
    routes: TiVec<Id<Route>, RouteDTO>,
    destinations: Vec<DestinationDTO>,
}

#[derive(Serialize, Deserialize)]
pub struct GameMap {
    pub cities: Cities,
    pub colors: Colors,

    pub destinations: TiVec<Id<Destination>, Destination>,
    pub routes: TiVec<Id<Route>, Route>,
}

#[derive(Debug, Error)]
pub enum GameMapCreationError {
    #[error(transparent)]
    CityNotFound(CityNotFoundError),
    #[error(transparent)]
    ColorNotFound(ColorNotFoundError),
}

impl GameMap {
    pub fn new(dto: GameMapDTO) -> Result<Self, Vec<GameMapCreationError>> {
        let GameMapDTO {
            cities,
            colors,
            destinations,
            routes,
        } = dto;
        let (destinations, destination_errors): (TiVec<_, _>, Vec<_>) = destinations
            .into_iter()
            .map(|destination| Destination::new(destination, &cities))
            .partition_result();
        let (routes, route_errors): (TiVec<_, _>, Vec<_>) = routes
            .into_iter()
            .map(|route| Route::new(route, &cities, &colors))
            .partition_result();
        let errors: Vec<GameMapCreationError> = destination_errors
            .into_iter()
            .map(DestinationCreationError::into)
            .chain(route_errors.into_iter().map(RouteCreationError::into))
            .collect();
        if errors.is_empty() {
            Result::Ok(Self {
                cities,
                colors,

                destinations,
                routes,
            })
        } else {
            Result::Err(errors)
        }
    }
}
