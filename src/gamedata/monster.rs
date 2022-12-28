use std::fmt::Display;
use std::rc::Rc;
use std::cmp::Ordering;

use crate::cardparse::carddata::CardData;
use crate::cardparse::enums::{AttackType, Ability};

#[derive(Debug)]
pub struct Monster<'a> {
    type_object: &'a CardData,
    key: u8,
    max_health: i32,
    health: i32,
    armor: i32,
    damage: i32,
    speed: i32,
    buffs_provided: Vec<Ability>,
}

impl<'a> Monster<'a> {
    pub fn new(type_object: &'a CardData, key: u8) -> Self {
        Self {
            type_object,
            key,
            max_health: type_object.health,
            health: type_object.health,
            armor: type_object.armor,
            damage: type_object.damage,
            speed: type_object.speed,
            buffs_provided: Vec::new(),
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

    pub fn set_health_new_max(&mut self, val: i32) {
        self.set_health(val);
        if self.health > self.max_health {
            self.max_health = self.health;
        }
    }

    pub fn get_max_health(&self) -> i32 {
        self.max_health
    }

    pub fn get_default_health(&self) -> i32 {
        self.type_object.health
    }

    pub fn is_alive(&self) -> bool {
        self.get_health() > 0
    }

    pub fn get_speed(&self) -> i32 {
        self.speed
    }

    pub fn get_default_speed(&self) -> i32 {
        self.type_object.speed
    }

    pub fn set_speed(&mut self, speed: i32) {
        self.speed = speed;
    }

    pub fn get_damage(&self) -> i32 {
        self.damage
    }

    pub fn get_default_damage(&self) -> i32 {
        self.type_object.damage
    }

    pub fn set_damage(&mut self, damage: i32) {
        self.damage = damage;
    }

    pub fn get_name(&self) -> &str {
        &self.type_object.name
    }

    pub fn get_key(&self) -> u8 {
        self.key
    }
    
    pub fn get_attack_type(&self) -> AttackType {
        self.type_object.attack_type
    }

    pub fn has_ability(&self, ability: Ability) -> bool {
        self.type_object.abilities.contains(&ability)
    }

    pub fn get_armor(&self) -> i32 {
        self.armor
    }

    pub fn get_default_armor(&self) -> i32 {
        self.type_object.armor
    }

    pub fn set_armor(&mut self, armor: i32) {
        self.armor = armor;
        if self.armor < 0 {
            self.armor = 0;
        }
    }
}

impl<'a> Display for Monster<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = format!("{} ({}/{})", self.get_name(), self.get_health(), self.get_max_health());
        if self.get_armor() > 0 {
            string.push_str(&format!(" [{}]", self.get_armor()));
        }
        write!(f, "{}", string)
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
