use std::collections::HashMap;

use crate::{cardparse::{carddata::CardData, cardparser::get_map}};
use crate::battles::battle::Battle;

#[derive(Debug)]
pub struct Registry {
    pub map: HashMap<String, CardData>,
}

impl From<&str> for Registry {
    fn from(filename: &str) -> Self {
        let map = get_map(filename).expect("file not found");
        map.into()
    }
}

impl From<HashMap<String, CardData>> for Registry {
    fn from(map: HashMap<String, CardData>) -> Self {
        Self {
            map,
        }
    }
}