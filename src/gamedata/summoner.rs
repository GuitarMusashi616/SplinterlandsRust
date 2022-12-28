use std::collections::HashSet;

use crate::cardparse::{carddata::CardData, enums::Ability};

#[derive(Debug)]
pub struct Summoner<'a> {
    type_object: &'a CardData,
}

impl<'a> Summoner<'a> {
    pub fn new(type_object: &'a CardData) -> Self {
        Self {
            type_object,
        }
    }

    pub fn get_abilities(&self) -> &HashSet<Ability> {
        &self.type_object.abilities
    }

    pub fn iter_abilities(&self) -> impl Iterator<Item = &Ability> {
        self.type_object.abilities.iter()
    }

    pub fn get_buffs(&self) -> Vec<Ability> {
        self
        .iter_abilities()
        .filter(|x| x.is_buff())
        .map(|x| x.clone())
        .collect()
    }

    pub fn get_debuffs(&self) -> Vec<Ability> {
        self
        .iter_abilities()
        .filter(|x| x.is_debuff())
        .map(|x| x.clone())
        .collect()
    }
}
