use std::rc::Rc;
use std::cmp::Ordering;

use crate::cardparse::carddata::CardData;

#[derive(Debug)]
pub struct Monster<'a> {
    type_object: &'a CardData,
    health: i32,
    armor: i32,
    pos: u8,
}

impl<'a> Monster<'a> {
    pub fn new(type_object: &'a CardData, pos: u8) -> Self {
        Self {
            type_object,
            health: type_object.health,
            armor: type_object.armor,
            pos,
        }
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn set_health(&mut self, val: i32) {
        self.health = val;
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
    
    pub fn get_pos(&self) -> u8 {
        self.pos
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