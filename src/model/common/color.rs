use arrayvec::ArrayString;
use serde::{Deserialize, Serialize};

pub type ColorName = ArrayString<8>;

#[derive(Serialize, Deserialize)]
pub struct Color {
    pub name: ColorName,
    pub neutral: bool,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
