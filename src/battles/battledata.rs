use std::collections::{HashMap, HashSet};

use rand::{seq::SliceRandom, thread_rng};

use crate::{gamedata::{monster::Monster, summoner::Summoner, registry::Registry}, cardparse::enums::{Ability, AttackType}};

use super::{monsterkey::MonsterKey, setpick::SetPick};

/// Wraps database related to battle
#[derive(Debug)]
pub struct BattleData<'a> {
    pub home_summ: Summoner<'a>,
    pub oppo_summ: Summoner<'a>,
    pub monsters: HashMap<MonsterKey, Monster<'a>>,
    pub home_alive: SetPick,
    pub oppo_alive: SetPick,
}

impl<'a> BattleData<'a> {
    pub fn new(reg: &'a Registry, home: Vec<&'a str>, oppo: Vec<&'a str>) -> Self {
        let (home_summ, home) = Self::vec_to_deck(reg, home);
        let (oppo_summ, oppo) = Self::vec_to_deck(reg, oppo);
        let (home_alive, oppo_alive) = Self::get_team_vecs(&home, &oppo);
        let monsters = Self::to_map(home, oppo);
        Self {
            home_summ,
            oppo_summ,
            monsters,
            home_alive: SetPick::new(&home_alive),
            oppo_alive: SetPick::new(&oppo_alive),
        }
    }

    pub fn get_team_vecs(home: &Vec<Monster<'a>>, oppo: &Vec<Monster<'a>>) -> (Vec<MonsterKey>, Vec<MonsterKey>) {
        let mut home_vec = Vec::new();
        let mut oppo_vec = Vec::new();
        for mons in home {
            home_vec.push(MonsterKey::Home(mons.get_key()));
        }
        for mons in oppo {
            oppo_vec.push(MonsterKey::Oppo(mons.get_key()));
        }
        (home_vec, oppo_vec)
    }

    pub fn vec_to_deck(reg: &'a Registry, cards: Vec<&'a str>) -> (Summoner<'a>, Vec<Monster<'a>>) {
        let mut monsters = Vec::new();
        for (i, &key) in cards.iter().skip(1).enumerate() {
            if let Some(carddata) = reg.map.get(key) {
                let monster = Monster::new(carddata, i as u8);
                monsters.push(monster);
            } else {
                panic!("card '{}' not found in registry", key);
            }
        }
        let summ = reg.map.get(cards[0]).expect("1st card must be a summoner");
        (Summoner::new(summ), monsters)
    }

    pub fn to_map(home: Vec<Monster<'a>>, oppo: Vec<Monster<'a>>) -> HashMap<MonsterKey, Monster<'a>> {
        let mut map = HashMap::new();
        for mons in home.into_iter() {
            let mk = MonsterKey::Home(mons.get_key() as u8);
            map.insert(mk, mons);
        }
        for mons in oppo.into_iter() {
            let mk = MonsterKey::Oppo(mons.get_key() as u8);
            map.insert(mk, mons);
        }
        map
    }

    // pub fn get_random_alive_enemy(&self, mk: &MonsterKey) -> Option<&MonsterKey> {
    //     let mut rng = thread_rng();
    //     match mk {
    //         MonsterKey::Home(_) => self.oppo_alive.choose(&mut rng),
    //         MonsterKey::Oppo(_) => self.home_alive.choose(&mut rng),
    //     }
    // }

    pub fn get(&self, mk: &MonsterKey) -> Option<&Monster<'a>> {
        self.monsters.get(mk)
    }

    pub fn get_pos(&self, mk: &MonsterKey) -> Option<u8> {
        if self.home_alive.contains(mk) {
            return self.home_alive.get_pos(mk).map(|x| *x as u8);
        }

        if self.oppo_alive.contains(mk) {
            return self.oppo_alive.get_pos(mk).map(|x| *x as u8);
        }

        None
    }

    pub fn handle_death(&mut self, mk: &MonsterKey) {
        let monster = self.get(mk).expect("mk is not part of battle");
        // remove from home_alive / oppo_alive
        if monster.get_health() > 0 {
            return
        }

        if self.home_alive.contains(mk) {
            self.home_alive.remove(mk);
            return;
        }

        if self.oppo_alive.contains(mk) {
            self.oppo_alive.remove(mk);
            return;
        }
    }

    pub fn deal_damage(&mut self, mk: &MonsterKey, dmg: i32) {
        let monster = self.monsters.get_mut(mk).expect("mk is not part of battle");
        let m_armor = monster.get_armor();
        if m_armor > 0 {
            monster.set_armor(m_armor - dmg);
            return
        }
        let m_health = monster.get_health();
        monster.set_health(m_health - dmg);
        self.handle_death(mk)
    }

    pub fn deal_true_damage(&mut self, mk: &MonsterKey, dmg: i32) {
        let monster = self.monsters.get_mut(mk).expect("mk is not part of battle");
        let m_health = monster.get_health();
        monster.set_health(m_health - dmg);
        self.handle_death(mk)
    }

    pub fn apply_summ_buffs(&mut self, mk: &MonsterKey, buffs: &[Ability]) {
        for buff in buffs {
            self.apply_summ_buff(mk, buff);
        }
    }
 
    pub fn apply_summ_buff(&mut self, mk: &MonsterKey, buff: &Ability) {
        let monster = self.monsters.get_mut(mk).unwrap();

        match buff {
            &Ability::Health(i) => {
                let health = monster.get_health();
                monster.set_health_new_max(health + Ability::extent_of(i));
            },
            &Ability::Armor(i) => {
                let armor = monster.get_armor();
                monster.set_armor(armor + Ability::extent_of(i));
            },
            &Ability::Speed(i) => {
                let speed = monster.get_speed();
                monster.set_speed(speed + Ability::extent_of(i));
            },
            &Ability::Melee(i) => {
                if monster.get_attack_type() != AttackType::Melee {
                    return;
                }
                let damage = monster.get_damage();
                monster.set_damage(damage + Ability::extent_of(i));
            },
            &Ability::Ranged(i) => {
                if monster.get_attack_type() != AttackType::Ranged {
                    return;
                }
                let damage = monster.get_damage();
                monster.set_damage(damage + Ability::extent_of(i));
            },
            &Ability::Magic(i) => {
                if monster.get_attack_type() != AttackType::Magic {
                    return;
                }
                let damage = monster.get_damage();
                monster.set_damage(damage + Ability::extent_of(i));
            },
            _ => (),
        }
    }

    /// Register only summoner buffs of health, armor, speed, melee, ranged, magic both + and -
    pub fn register_all_team_buffs(&mut self) {
        let home_alive: Vec<MonsterKey> = self.home_alive.iter().map(|x| x.clone()).collect();
        let oppo_alive: Vec<MonsterKey> = self.oppo_alive.iter().map(|x| x.clone()).collect();

        let home_buffs = self.home_summ.get_buffs();
        let home_debuffs = self.oppo_summ.get_debuffs();
        let oppo_buffs = self.oppo_summ.get_buffs();
        let oppo_debuffs = self.home_summ.get_debuffs();

        self.register_buffs_on_team(&home_alive, &home_buffs);
        self.register_buffs_on_team(&oppo_alive, &oppo_buffs);
        self.register_buffs_on_team(&home_alive, &home_debuffs);
        self.register_buffs_on_team(&oppo_alive, &oppo_debuffs);

        self.set_minimum_monster_values();
    }

    pub fn register_buffs_on_team(&mut self, alive: &[MonsterKey], abilities: &[Ability]) {
        alive.iter().for_each(|mk| {
            self.apply_summ_buffs(mk, abilities);
        })
    }

    pub fn set_minimum_monster_values(&mut self) {
        for monster in self.monsters.values_mut() {
            if monster.get_damage() <= 0 && monster.get_default_damage() > 0 {
                monster.set_damage(1);
            }
            if monster.get_health() <= 0 && monster.get_default_health() > 0 {
                monster.set_health(1);
            }
            if monster.get_speed() <= 0 && monster.get_default_speed() > 0 {
                monster.set_speed(1);
            }
        }
    }


    /// Register individual buffs
    pub fn register_buffs(&mut self, mk: &MonsterKey) {

    }

}

#[cfg(test)]
mod tests {
    use crate::{gamedata::registry::Registry, battles::monsterkey::MonsterKey};

    use super::BattleData;

    #[test]
    fn test_get_enemy() {
        // populate battledata
        let mut reg = Registry::from("assets/cards.csv");
        let home = vec!["Drake of Arnak", "Goblin Shaman", "Fire Beetle"];
        let oppo = vec!["Pyre", "Spineback Turtle", "Kobold Bruiser"];
        let mut bd = BattleData::new(&reg, home, oppo);
        let exp = vec![MonsterKey::Home(0), MonsterKey::Home(1), MonsterKey::Home(2)];

        // assert_eq!(res, exp);
    }
}
