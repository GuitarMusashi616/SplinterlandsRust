use std::rc::Rc;
use std::cmp::Ordering;

use crate::cardparse::carddata::CardData;
use crate::cardparse::enums::{AttackType, Ability};

#[derive(Debug)]
pub struct Monster<'a> {
    type_object: &'a CardData,
    health: i32,
    armor: i32,
    key: u8,
}

impl<'a> Monster<'a> {
    pub fn new(type_object: &'a CardData, key: u8) -> Self {
        Self {
            type_object,
            health: type_object.health,
            armor: type_object.armor,
            key,
        }
    }
    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn set_health(&mut self, val: i32) {
        self.health = val;
        if self.health < 0 {
            self.health = 0
        }
    }

    pub fn get_max_health(&self) -> i32 {
        self.type_object.health
    }

    pub fn is_alive(&self) -> bool {
        self.get_health() > 0
    }

    pub fn get_speed(&self) -> i32 {
        self.type_object.speed
    }

    pub fn get_damage(&self) -> i32 {
        self.type_object.damage
    }

    pub fn get_name(&self) -> &str {
        &self.type_object.name
    }

    pub fn get_key(&self) -> u8 {
        self.key
    }
    
    pub fn get_attack_type(&self) -> &AttackType {
        &self.type_object.attack_type
    }

    pub fn has_ability(&self, ability: Ability) -> bool {
        self.type_object.abilities.contains(&ability)
    }

    pub fn get_armor(&self) -> i32 {
        self.armor
    }

    pub fn get_max_armor(&self) -> i32 {
        self.type_object.armor
    }

    pub fn set_armor(&mut self, armor: i32) {
        self.armor = armor;
        if self.armor < 0 {
            self.armor = 0;
        }
    }
}

// impl Ord for Monster<'a> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.get_speed().cmp(&other.get_speed())
//     }
// }

// impl PartialOrd for Monster {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for Monster {
//     fn eq(&self, other: &Self) -> bool {
//         self.get_speed() == other.get_speed()
//     }
// }

// impl Eq for Monster {}