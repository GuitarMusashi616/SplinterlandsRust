use std::collections::{BinaryHeap, HashMap};

use crate::{battles::battledata::BattleData, gamedata::{summoner::Summoner, monster::Monster, registry::Registry}};

use super::{roundrobin::RoundRobin, monsterspeed::MonsterSpeed, roundrobin2::RoundRobinIter};



#[derive(Debug)]
pub struct Battle<'a> {
    battledata: BattleData<'a>,
}


impl<'a> Battle<'a> {
    pub fn new(reg: &'a Registry, home: Vec<&'a str>, oppo: Vec<&'a str>) -> Self {
        let battledata = BattleData::new(reg, home, oppo);
        Self {
            battledata
        }
    }

    pub fn round(&self) {
        let mks = MonsterSpeed::get_vec(&self.battledata);
        for mk in RoundRobinIter::new(mks) {
            dbg!(mk);
        }
    }

    pub fn turn() {

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_battle() {
        // tryna make it use keys to get order and battle?
        // gotta choose based on speed
        // next choose based on team of monster
        // then affect choice based on global cooldowns
        // battle.get_next()
    }
}