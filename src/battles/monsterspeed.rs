use std::cmp::Ordering;

use super::{monsterkey::MonsterKey, battledata::BattleData};

// A class for bin heap
pub struct MonsterSpeed {
    mk: MonsterKey,
    speed: u8,
}


impl MonsterSpeed {
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