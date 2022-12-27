use std::cmp::Ordering;

use crate::gamedata::{registry::Registry, monster::Monster};

use super::{monsterkey::MonsterKey, battledata::BattleData, roundrobin::RoundRobin};

// A class for bin heap
#[derive(Debug, Clone, Hash)]
pub struct MonsterSpeed {
    pub mk: MonsterKey,
    pub speed: u8,
}


impl MonsterSpeed {
    pub fn new(mk: MonsterKey, speed: u8) -> Self {
        Self {
            mk,
            speed
        }
    }

    pub fn get_vec(bd: &BattleData) -> Vec<Self> {
        let vec = bd.monsters
        .iter()
        .filter(|(mk, mons)| mons.get_health() > 0)
        .map(|(mk, mons)| {
            Self::new(*mk, mons.get_speed() as u8)
        }).collect();
        vec
    }
}

impl Ord for MonsterSpeed {
    fn cmp(&self, other: &Self) -> Ordering {
        self.speed.cmp(&other.speed)
    }
}

impl PartialOrd for MonsterSpeed {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MonsterSpeed {
    fn eq(&self, other: &Self) -> bool {
        self.speed == other.speed
    }
}

impl Eq for MonsterSpeed {}