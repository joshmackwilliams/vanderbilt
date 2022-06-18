use super::color::ColorId;
use super::player::PlayerId;
use super::{city::CityId, game::GameCreationError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteId(pub u16);

#[derive(Serialize, Deserialize)]
pub struct RouteDTO {
    pub from: String,
    pub to: String,
    pub length: usize,
    pub color: String,
    pub double_color: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RouteDouble {
    pub color: ColorId,
    pub owner: Option<PlayerId>,
}

#[derive(Serialize, Deserialize)]
pub struct Route {
    pub from: CityId,
    pub to: CityId,
    pub length: usize,
    pub color: ColorId,
    pub double: Option<RouteDouble>,
    pub owner: Option<PlayerId>,
}

impl Route {
    pub fn new(
        dto: RouteDTO,
        cities_by_name: &HashMap<String, CityId>,
        colors_by_name: &HashMap<String, ColorId>,
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
            .unwrap_or_else(|| Result::Err(GameCreationError::ColorNotFound(to)))?;
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
