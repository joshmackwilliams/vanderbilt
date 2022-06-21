use super::city::{City, CityName};
use super::color::ColorName;
use super::player::PlayerId;
use super::{color::Color, game::GameCreationError};
use crate::model::id::Id;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct RouteDTO {
    pub from: CityName,
    pub to: CityName,
    pub length: usize,
    pub color: ColorName,
    pub double_color: Option<ColorName>,
}

#[derive(Serialize, Deserialize)]
pub struct RouteDouble {
    pub color: Id<Color>,
    pub owner: Option<Id<PlayerId>>,
}

#[derive(Serialize, Deserialize)]
pub struct Route {
    pub from: Id<City>,
    pub to: Id<City>,
    pub length: usize,
    pub color: Id<Color>,
    pub double: Option<RouteDouble>,
    pub owner: Option<Id<PlayerId>>,
}

impl Route {
    pub fn new(
        dto: RouteDTO,
        cities_by_name: &HashMap<CityName, Id<City>>,
        colors_by_name: &HashMap<ColorName, Id<Color>>,
    ) -> Result<Self, GameCreationError> {
        let RouteDTO {
            from,
            to,
            length,
            color,
            double_color,
        } = dto;
        let from = *cities_by_name
            .get(&from)
            .map(Result::Ok)
            .unwrap_or_else(|| Result::Err(GameCreationError::CityNotFound(from)))?;
        let to = *cities_by_name
            .get(&to)
            .map(Result::Ok)
            .unwrap_or_else(|| Result::Err(GameCreationError::CityNotFound(to)))?;
        let color = *colors_by_name
            .get(&color)
            .map(Result::Ok)
            .unwrap_or_else(|| Result::Err(GameCreationError::ColorNotFound(color)))?;
        let double_color = double_color
            .map(|color_name| {
                colors_by_name
                    .get(&color_name)
                    .map(|color| Result::Ok(*color))
                    .unwrap_or_else(|| Result::Err(GameCreationError::ColorNotFound(color_name)))
            })
            .transpose()?;
        let double = double_color.map(|color| RouteDouble {
            color,
            owner: Option::None,
        });
        Result::Ok(Self {
            from,
            to,
            length,
            color,
            double,
            owner: Option::None,
        })
    }
}
