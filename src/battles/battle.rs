use std::collections::{BinaryHeap, HashMap};

use crate::{battles::{battledata::BattleData, targeting, attacking}, gamedata::{summoner::Summoner, monster::Monster, registry::Registry}};

use super::{roundrobin::RoundRobin, monsterspeed::MonsterSpeed, roundrobiniter::RoundRobinIter, monsterkey::MonsterKey};



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

    pub fn is_over(&self) -> bool {
        let mut home_count = 0;
        let mut oppo_count = 0;
        for (mk, monster) in self.battledata.monsters.iter() {
            if monster.is_alive() {
                match mk {
                    MonsterKey::Home(_) => {home_count += 1},
                    MonsterKey::Oppo(_) => {oppo_count += 1}
                }
                if home_count > 0 && oppo_count > 0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn game(&mut self) {
        let mut i = 1;
        while !self.is_over() {
            println!("Round {}:\n{:?}\n{:?}\n\n", i, self.battledata.home_alive, self.battledata.oppo_alive);
            self.round();
            i += 1;
        }
    }
    
    pub fn round(&mut self) {
        let mss = MonsterSpeed::get_vec(&self.battledata);
        for ms in RoundRobinIter::new(mss) {
            let tk = targeting::target_for(&self.battledata, &ms.mk);
            if tk.is_none() {
                continue;
            }
            let tk = tk.unwrap();
            attacking::attack(&mut self.battledata, &ms.mk, &tk);
        }
    }

    pub fn turn() {

    }
}

#[cfg(test)]
mod tests {
    use crate::{gamedata::registry::Registry, battles::{monsterkey::MonsterKey, attacking, roundrobiniter::RoundRobinIter, monsterspeed::MonsterSpeed}};

    use super::Battle;
    use crate::battles::targeting;

    #[test]
    fn test_melee_target() {
        let reg = Registry::from("assets/cards.csv");

        let home = vec!["Pyre", "Living Lava", "Kobold Bruiser"];
        let oppo = vec!["Bortus", "Serpent of Eld", "Sniping Narwhal"];
        
        let mut battle = Battle::new(&reg, home, oppo);

        let mk = MonsterKey::Home(0);
        let player = battle.battledata.get(&mk).unwrap();
        assert_eq!(player.get_name(), "Living Lava");

        let enemy = targeting::target_for_melee(&battle.battledata, &mk).unwrap();
        let enemy_mons = battle.battledata.get(&enemy).unwrap();
        assert_eq!(enemy_mons.get_name(), "Serpent of Eld");

    }

    #[test]
    fn test_melee_not_in_1st_pos() {
        let reg = Registry::from("assets/cards.csv");

        let home = vec!["Pyre", "Living Lava", "Kobold Bruiser"];
        let oppo = vec!["Bortus", "Serpent of Eld", "Sniping Narwhal"];
        
        let mut battle = Battle::new(&reg, home, oppo);

        let pk = &MonsterKey::Home(1);
        let target = targeting::target_for_melee(&battle.battledata, pk);
        assert_eq!(target, None);
    }

    #[test]
    fn test_melee_reach() {
        let reg = Registry::from("assets/cards.csv");

        let home = vec!["Pyre", "Living Lava", "Magma Troll", "Kobold Bruiser"];
        let oppo = vec!["Bortus", "Serpent of Eld", "Sniping Narwhal"];
        
        let mut battle = Battle::new(&reg, home, oppo);

        let pk = &MonsterKey::Home(1);
        let target = targeting::target_for_melee(&battle.battledata, pk).unwrap();
        assert_eq!(target, MonsterKey::Oppo(0));
    }

    #[test]
    fn test_melee_armor_attack() {
        let reg = Registry::from("assets/cards.csv");

        let home = vec!["Pyre", "Living Lava", "Magma Troll", "Kobold Bruiser"];
        let oppo = vec!["Bortus", "Serpent of Eld", "Sniping Narwhal"];
        
        let mut battle = Battle::new(&reg, home, oppo);

        let mk = &MonsterKey::Home(0);
        let tk = targeting::target_for_melee(&battle.battledata, mk).unwrap();

        attacking::attack_melee_or_ranged(&mut battle.battledata, mk, &tk);

        let target = battle.battledata.get(&tk).unwrap();
        assert_eq!(target.get_armor(), 0);
        assert_eq!(target.get_health(), 5);
    }

    #[test]
    fn test_ranged_target() {
        let reg = Registry::from("assets/cards.csv");

        let home = vec!["Pyre", "Living Lava", "Magma Troll", "Kobold Bruiser", "Goblin Fireballer"];
        let oppo = vec!["Bortus", "Serpent of Eld", "Sniping Narwhal"];
        
        let mut battle = Battle::new(&reg, home, oppo);

        let mk = &MonsterKey::Home(3);
        let target = targeting::target_for_ranged(&battle.battledata, mk).unwrap();
        assert!(target == MonsterKey::Oppo(0) || target == MonsterKey::Oppo(1));
    }

    #[test]
    fn test_ranged_in_1st_pos() {
        let reg = Registry::from("assets/cards.csv");

        let home = vec!["Pyre", "Goblin Fireballer"];
        let oppo = vec!["Bortus", "Serpent of Eld", "Sniping Narwhal"];
        
        let mut battle = Battle::new(&reg, home, oppo);

        let mk = &MonsterKey::Home(0);
        let target = targeting::target_for_ranged(&battle.battledata, mk);
        assert_eq!(target, None);
    }

    #[test]
    fn test_magic_target() {
        let reg = Registry::from("assets/cards.csv");

        let home = vec!["Pyre", "Living Lava", "Magma Troll", "Kobold Bruiser", "Goblin Fireballer"];
        let oppo = vec!["Bortus", "Serpent of Eld", "Sniping Narwhal", "Ice Pixie"];
        
        let mut battle = Battle::new(&reg, home, oppo);

        let mk = &MonsterKey::Oppo(2);
        let target = targeting::target_for_magic(&battle.battledata, mk).unwrap();
        assert!(target == MonsterKey::Home(0) || target == MonsterKey::Home(1) || target == MonsterKey::Home(2) || target == MonsterKey::Home(3));
    }

    #[test]
    fn test_magic_attack() {
        let reg = Registry::from("assets/cards.csv");

        let home = vec!["Pyre", "Living Lava", "Magma Troll", "Kobold Bruiser", "Goblin Fireballer"];
        let oppo = vec!["Bortus", "Serpent of Eld", "Sniping Narwhal", "Ice Pixie"];
        
        let mut battle = Battle::new(&reg, home, oppo);

        let mk = &MonsterKey::Oppo(2);
        let tk = &MonsterKey::Home(0);

        attacking::attack_magic(&mut battle.battledata, mk, tk);
        let target = battle.battledata.get(tk).unwrap();
        assert_eq!(target.get_health(), 5);
        assert_eq!(target.get_armor(), 2);
    }

    #[test]
    fn test_skip_dead() {
        let reg = Registry::from("assets/cards.csv");

        let home = vec!["Pyre", "Living Lava", "Magma Troll", "Kobold Bruiser", "Goblin Fireballer"];
        let oppo = vec!["Bortus", "Serpent of Eld", "Sniping Narwhal", "Ice Pixie"];
        
        let mut battle = Battle::new(&reg, home, oppo);
        let dmg = 1000;
        
        for i in 0..=2 {
            let hk = MonsterKey::Home(i);
            let ok = MonsterKey::Oppo(i);
            battle.battledata.deal_true_damage(&hk, dmg);
            battle.battledata.deal_true_damage(&ok, dmg);
        }

        let rem: Vec<_> = RoundRobinIter::new(MonsterSpeed::get_vec(&battle.battledata)).collect();
        assert_eq!(rem.len(), 1);
        let guy = battle.battledata.monsters.get(&rem[0].mk).unwrap();
        assert_eq!(guy.get_name(), "Goblin Fireballer");
    }

    #[test]
    fn test_dont_target_dead() {
        let reg = Registry::from("assets/cards.csv");

        let home = vec!["Pyre", "Living Lava", "Magma Troll", "Kobold Bruiser", "Goblin Fireballer"];
        let oppo = vec!["Bortus", "Serpent of Eld", "Sniping Narwhal", "Ice Pixie"];

        let mut battle = Battle::new(&reg, home, oppo);

        battle.battledata.deal_true_damage(&MonsterKey::Home(0), 1000);
        battle.battledata.deal_true_damage(&MonsterKey::Home(2), 1000);
        battle.battledata.deal_true_damage(&MonsterKey::Oppo(1), 1000);

        let ms_vec = MonsterSpeed::get_vec(&battle.battledata);
        for ms in RoundRobinIter::new(ms_vec) {
            let tk = targeting::target_for(&battle.battledata, &ms.mk);
            if let Some(tk) = tk {
                assert_ne!(ms.mk, tk);  // no self targeting
                let target = battle.battledata.get(&tk).unwrap();
                assert!(target.get_health() > 0);  // no dead targets
            }
        }
    }

    #[test]
    fn test_pos_after_death() {
        let reg = Registry::from("assets/cards.csv");

        let home = vec!["Pyre", "Living Lava", "Magma Troll", "Kobold Bruiser", "Goblin Fireballer"];
        let oppo = vec!["Bortus", "Serpent of Eld", "Sniping Narwhal", "Ice Pixie"];

        let mut battle = Battle::new(&reg, home, oppo);

        battle.battledata.deal_true_damage(&MonsterKey::Home(0), 1000);
        battle.battledata.deal_true_damage(&MonsterKey::Home(2), 1000);
        battle.battledata.deal_true_damage(&MonsterKey::Oppo(1), 1000);

        let troll = &MonsterKey::Home(1);
        let goblin = &MonsterKey::Home(3);
        let pixie = &MonsterKey::Oppo(2);

        // println!("home rem: {:?}", battle.battledata.home_alive);
        // println!("oppo rem: {:?}", battle.battledata.oppo_alive);

        assert_eq!(battle.battledata.get_pos(troll).unwrap(), 0);
        assert_eq!(battle.battledata.get_pos(goblin).unwrap(), 1);
        assert_eq!(battle.battledata.get_pos(pixie).unwrap(), 1);
    }
}