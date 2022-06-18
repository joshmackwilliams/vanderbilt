use super::city::{City, CityId};
use super::color::{Color, ColorId};
use super::destination::{Destination, DestinationDTO};
use super::route::{Route, RouteDTO};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct GameDTO {
    pub cities: Vec<City>,
    pub colors: Vec<Color>,
    pub routes: Vec<RouteDTO>,
    pub destinations: Vec<DestinationDTO>,
}

#[derive(Serialize, Deserialize)]
pub struct GameCommon {
    pub cities: Vec<City>,
    pub colors: Vec<Color>,
    pub destinations: Vec<Destination>,
    pub routes: Vec<Route>,

    pub cities_by_name: HashMap<String, CityId>,
    pub colors_by_name: HashMap<String, ColorId>,
}

pub enum GameCreationError {
    CityNotFound(String),
    ColorNotFound(String),
}

impl GameCommon {
    pub fn new(
        cities: Vec<City>,
        colors: Vec<Color>,
        destinations: Vec<DestinationDTO>,
        routes: Vec<RouteDTO>,
    ) -> Result<Self, GameCreationError> {
        let cities_by_name = cities
            .iter()
            .enumerate()
            .map(|(id, city)| (city.name.clone(), CityId(id.try_into().unwrap())))
            .collect();
        let colors_by_name = colors
            .iter()
            .enumerate()
            .map(|(id, color)| (color.name.clone(), ColorId(id.try_into().unwrap())))
            .collect();
        let destinations = destinations
            .into_iter()
            .map(|destination| Destination::new(destination, &cities_by_name))
            .collect::<Result<Vec<Destination>, GameCreationError>>()?;
        let routes = routes
            .into_iter()
            .map(|route| Route::new(route, &cities_by_name, &colors_by_name))
            .collect::<Result<Vec<Route>, GameCreationError>>()?;
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
