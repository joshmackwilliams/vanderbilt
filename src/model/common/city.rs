use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CityId(pub u16);

#[derive(Serialize, Deserialize)]
pub struct City {
    pub name: String,
    pub x: f32, // 0.0 to 1.0, indicating position on the map
    pub y: f32,
}
