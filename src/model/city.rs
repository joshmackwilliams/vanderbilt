use std::collections::HashMap;

use arrayvec::ArrayString;
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};
use typed_index_collections::TiVec;

use super::id::Id;

pub type CityName = ArrayString<16>;

#[derive(Serialize, Deserialize, CopyGetters, Getters)]
pub struct City {
    #[getset(get = "pub")]
    name: CityName,
    #[getset(get_copy = "pub")]
    x: u8, // 0.0 to 1.0, indicating position on the map
    #[getset(get_copy = "pub")]
    y: u8,
}

#[derive(Getters)]
pub struct Cities {
    #[getset(get = "pub")]
    cities: TiVec<Id<City>, City>,
    by_name: HashMap<CityName, Id<City>>,
}

impl Cities {
    pub fn by_name(&self, name: &CityName) -> Option<Id<City>> {
        self.by_name.get(name).copied()
    }
}

impl From<TiVec<Id<City>, City>> for Cities {
    fn from(cities: TiVec<Id<City>, City>) -> Self {
        let by_name = cities
            .iter_enumerated()
            .map(|(id, city)| (city.name, id))
            .collect();
        Self { cities, by_name }
    }
}

impl Serialize for Cities {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.cities.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Cities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        TiVec::deserialize(deserializer).map(Cities::from)
    }
}
