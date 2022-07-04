use super::city::{Cities, City, CityName};
use super::color::{ColorName, Colors};
use super::{color::Color, game_map::GameMapCreationError};
use crate::errors::{CityNotFoundError, ColorNotFoundError};
use crate::model::id::Id;
use derive_more::From;
use getset::CopyGetters;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct RouteDTO {
    pub from: CityName,
    pub to: CityName,
    pub length: usize,
    pub color: ColorName,
    pub double_color: Option<ColorName>,
}

#[derive(Error, Debug, From)]
pub enum RouteCreationError {
    #[error(transparent)]
    CityNotFound(CityNotFoundError),
    #[error(transparent)]
    ColorNotFound(ColorNotFoundError),
}

impl From<RouteCreationError> for GameMapCreationError {
    fn from(e: RouteCreationError) -> Self {
        match e {
            RouteCreationError::CityNotFound(e) => Self::CityNotFound(e),
            RouteCreationError::ColorNotFound(e) => Self::ColorNotFound(e),
        }
    }
}

#[derive(Serialize, Deserialize, CopyGetters)]
pub struct Route {
    #[getset(get_copy = "pub")]
    from: Id<City>,
    #[getset(get_copy = "pub")]
    to: Id<City>,
    #[getset(get_copy = "pub")]
    length: usize,
    #[getset(get_copy = "pub")]
    color: Id<Color>,
    #[getset(get_copy = "pub")]
    double_color: Option<Id<Color>>,
}

impl Route {
    pub fn new(
        dto: RouteDTO,
        cities: &Cities,
        colors: &Colors,
    ) -> Result<Self, RouteCreationError> {
        let RouteDTO {
            from,
            to,
            length,
            color,
            double_color,
        } = dto;
        let from = cities
            .by_name(&from)
            .ok_or_else(|| CityNotFoundError::from(from))?;
        let to = cities
            .by_name(&to)
            .ok_or_else(|| CityNotFoundError::from(to))?;
        let color = colors
            .by_name(&color)
            .ok_or_else(|| ColorNotFoundError::from(color))?;
        let double_color = double_color
            .map(|color_name| {
                colors
                    .by_name(&color_name)
                    .ok_or_else(|| ColorNotFoundError::from(color_name))
            })
            .transpose()?;
        Result::Ok(Self {
            from,
            to,
            length,
            color,
            double_color,
        })
    }
}
