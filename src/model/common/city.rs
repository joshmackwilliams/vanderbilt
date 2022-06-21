use arrayvec::ArrayString;
use serde::{Deserialize, Serialize};

pub type CityName = ArrayString<16>;

#[derive(Serialize, Deserialize)]
pub struct City {
    pub name: CityName,
    pub x: u8, // 0.0 to 1.0, indicating position on the map
    pub y: u8,
}
