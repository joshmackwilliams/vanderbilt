use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorId(pub u16);

#[derive(Serialize, Deserialize)]
pub struct Color {
    pub name: String,
    pub neutral: bool,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
