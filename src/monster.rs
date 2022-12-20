use std::rc::Rc;
use std::cmp::Ordering;

use crate::carddata::CardData;

#[derive(Debug)]
pub struct Monster {
    type_object: Rc<CardData>,
    health: i32,
    armor: i32,
}

impl Monster {
    pub fn new(type_object: &Rc<CardData>) -> Self {
        Self {
            type_object: Rc::clone(type_object),
            health: type_object.health,
            armor: type_object.armor,
        }
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn is_alive(&self) -> bool {
        self.get_health() > 0
    }

    pub fn get_speed(&self) -> i32 {
        self.type_object.speed
    }

    pub fn get_name(&self) -> &str {
        &self.type_object.name
    }
}

impl Ord for Monster {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_speed().cmp(&other.get_speed())
    }
}

impl PartialOrd for Monster {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Monster {
    fn eq(&self, other: &Self) -> bool {
        self.get_speed() == other.get_speed()
    }
}

impl Eq for Monster {}