use crate::model::city::CityName;
use derive_more::From;
use thiserror::Error;

#[derive(Error, Debug, From)]
#[error("city not found: {name}")]
pub struct CityNotFoundError {
    name: CityName,
}
