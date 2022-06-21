use super::city::{City, CityName};
use super::color::{Color, ColorName};
use super::destination::{Destination, DestinationDTO};
use super::route::{Route, RouteDTO};
use crate::model::id::Id;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
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

pub enum GameCreationError {
    CityNotFound(CityName),
    ColorNotFound(ColorName),
}

impl Display for GameCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CityNotFound(city) => write!(f, "City not found: {}", city),
            Self::ColorNotFound(color) => write!(f, "Color not found: {}", color),
        }
    }
}

impl GameCommon {
    pub fn new(dto: GameDTO) -> Result<Self, GameCreationError> {
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
        let destinations = destinations
            .into_iter()
            .map(|destination| Destination::new(destination, &cities_by_name))
            .collect::<Result<TiVec<Id<Destination>, Destination>, GameCreationError>>()?;
        let routes = routes
            .into_iter()
            .map(|route| Route::new(route, &cities_by_name, &colors_by_name))
            .collect::<Result<TiVec<Id<Route>, Route>, GameCreationError>>()?;
        Result::Ok(Self {
            cities,
            colors,
            destinations,
            routes,

            cities_by_name,
            colors_by_name,
        })
    }
}
