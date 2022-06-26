use crate::model::color::ColorName;
use derive_more::From;
use thiserror::Error;

#[derive(Error, Debug, From)]
#[error("color not found: {name}")]
pub struct ColorNotFoundError {
    name: ColorName,
}
