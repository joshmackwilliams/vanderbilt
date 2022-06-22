use super::city::{City, CityName};
use super::color::{Color, ColorName};
use super::destination::{Destination, DestinationDTO};
use super::route::{Route, RouteDTO};
use crate::model::id::Id;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use typed_index_collections::TiVec;

#[derive(Serialize, Deserialize)]
pub struct GameDTO {
    pub cities: TiVec<Id<City>, City>,
    pub colors: TiVec<Id<Color>, Color>,
    pub routes: TiVec<Id<Route>, RouteDTO>,
    pub destinations: Vec<DestinationDTO>,
}

#[derive(Serialize, Deserialize)]
pub struct GameCommon {
    pub cities: TiVec<Id<City>, City>,
    pub colors: TiVec<Id<Color>, Color>,
    pub destinations: TiVec<Id<Destination>, Destination>,
    pub routes: TiVec<Id<Route>, Route>,

    pub cities_by_name: HashMap<CityName, Id<City>>,
    pub colors_by_name: HashMap<ColorName, Id<Color>>,
}

#[derive(Debug, Error)]
pub enum GameCreationError {
    #[error("City not found: {0}")]
    CityNotFoundError(CityName),
    #[error("Color not found: {0}")]
    ColorNotFoundError(ColorName),
}

impl GameCommon {
    pub fn new(dto: GameDTO) -> Result<Self, Vec<GameCreationError>> {
        let GameDTO {
            cities,
            colors,
            destinations,
            routes,
        } = dto;
        let cities_by_name = cities
            .iter_enumerated()
            .map(|(id, city)| (city.name, id))
            .collect();
        let colors_by_name = colors
            .iter_enumerated()
            .map(|(id, color)| (color.name, id))
            .collect();
        let (destinations, mut errors): (TiVec<_, _>, Vec<_>) = destinations
            .into_iter()
            .map(|destination| Destination::new(destination, &cities_by_name))
            .partition_result();
        let (routes, mut route_errors): (TiVec<_, _>, Vec<_>) = routes
            .into_iter()
            .map(|route| Route::new(route, &cities_by_name, &colors_by_name))
            .partition_result();
        errors.append(&mut route_errors);
        if errors.is_empty() {
            Result::Ok(Self {
                cities,
                colors,
                destinations,
                routes,

                cities_by_name,
                colors_by_name,
            })
        } else {
            Result::Err(errors)
        }
    }
}
