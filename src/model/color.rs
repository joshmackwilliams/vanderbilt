use std::collections::HashMap;

use super::id::Id;
use arrayvec::ArrayString;
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};
use typed_index_collections::TiVec;

pub type ColorName = ArrayString<8>;

#[derive(Serialize, Deserialize, CopyGetters, Getters)]
pub struct Color {
    #[getset(get = "pub")]
    name: ColorName,
    #[getset(get_copy = "pub")]
    neutral: bool,
    #[getset(get_copy = "pub")]
    r: u8,
    #[getset(get_copy = "pub")]
    g: u8,
    #[getset(get_copy = "pub")]
    b: u8,
}

#[derive(Getters)]
pub struct Colors {
    #[getset(get = "pub")]
    colors: TiVec<Id<Color>, Color>,
    by_name: HashMap<ColorName, Id<Color>>,
}

impl Colors {
    pub fn by_name(&self, name: &ColorName) -> Option<Id<Color>> {
        self.by_name.get(name).copied()
    }
}

impl From<TiVec<Id<Color>, Color>> for Colors {
    fn from(colors: TiVec<Id<Color>, Color>) -> Self {
        let by_name = colors
            .iter_enumerated()
            .map(|(id, color)| (color.name, id))
            .collect();
        Self { colors, by_name }
    }
}

impl<'de> Deserialize<'de> for Colors {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        TiVec::deserialize(deserializer).map(Colors::from)
    }
}

impl Serialize for Colors {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.colors.serialize(serializer)
    }
}
