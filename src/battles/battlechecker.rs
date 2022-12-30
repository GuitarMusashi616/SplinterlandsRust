use std::collections::HashSet;

use crate::gamedata::{registry::Registry, monster::Monster};

use super::{battle::Battle, monsterkey::MonsterKey, targeting};

/// Wraps a battle and makes testing easier
pub struct BattleChecker<'a> {
    battle: Battle<'a>,
}

impl<'a> BattleChecker<'a> {
    /// Will not work if same monster on separate teams
    pub fn new(reg: &'a Registry, home: &'a Vec<&'a str>, oppo: &'a Vec<&'a str>) -> Self {
        let battle = Battle::new(reg, home, oppo);
        if Self::has_same_monster_on_both_teams(home, oppo) {
            panic!("does not support same monster on both teams\nhome: {:?}, oppo: {:?}", home, oppo);
        }
        
        Self {
            battle,
        }
    }

    fn has_same_monster_on_both_teams(home: &'a Vec<&'a str>, oppo: &'a Vec<&'a str>) -> bool {
        // put all oppo into a hash set
        let mut enemies = HashSet::new();
        for name in oppo {
            enemies.insert(name);
        }
        for name in home {
            if enemies.contains(name) {
                return true;
            }
        }
        false
    }

    pub fn assert_target(&self, subject: &str, target: &str) {
        // pull out the monster and its mk
        // get the targeting of the mk as tk and get its name
        // assert that the name of the targeting of mk is the same as target
        let (mk_sub, mons_sub) = self.get_mk_and_monster_by_name(subject).expect(&format!("{} could not be found", subject));
        // let (mk_tar, mons_tar) = self.get_mk_and_monster_by_name(target).expect(&format!("{} could not be found", target));
        let mk_tar = targeting::target_for(&self.battle.battledata, mk_sub);
        let mk_tar = mk_tar.map(|x| self.battle.battledata.get(&x).expect(&format!("{:?} not found", x)).get_name());
        let tar_name = mk_tar.expect(&format!("{} targeted {:?} instead of {}", mons_sub.get_name(), mk_tar, target));
        if tar_name != target { 
            panic!("{} targeted {} instead of {}", mons_sub.get_name(), tar_name, target);
        }
    }

    pub fn get_mk_and_monster_by_name(&self, name: &str) -> Option<(&MonsterKey, &Monster)>{
        let res = self.battle.battledata.monsters.iter().find(|(mk, mons)| mons.get_name() == name);
        res
    }
}