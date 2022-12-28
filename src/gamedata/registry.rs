use std::{collections::HashMap, ops::Deref};

use crate::{cardparse::{carddata::CardData, cardparser::get_map, enums::Element}};
use crate::battles::battle::Battle;

#[derive(Debug)]
pub struct Registry {
    pub map: HashMap<String, CardData>,
}

impl<'a> Registry {
    // pub fn get_element(&'a self, elem: Element) -> Vec<&'a str> {
    //     let res = self.map.iter().filter_map(|(name, card)| {
    //         if card.element == elem {
    //             Some(name.as_ref())
    //         } else {
    //             None
    //         }
    //     }).collect();
    //     res
    // }

    pub fn query(&'a self, filter: impl Fn(&CardData) -> bool) -> Vec<&'a str> {
        self.map.iter().filter_map(|(name, card)| {
            if filter(card) {
                Some(name.as_ref())
            } else {
                None
            }
        }).collect()
    }

    pub fn filter(&'a self, filter: impl Fn(&CardData) -> bool) -> Vec<(&'a str, &'a CardData)> {
        self.map.iter().filter_map(|(name, card)| {
            if filter(card) {
                return Some((name.as_ref(), card))
            }
            None
        }).collect()
    }
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
