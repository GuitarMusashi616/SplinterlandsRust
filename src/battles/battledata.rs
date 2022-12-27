use std::collections::{HashMap, HashSet};

use rand::{seq::SliceRandom, thread_rng};

use crate::gamedata::{monster::Monster, summoner::Summoner, registry::Registry};

use super::monsterkey::MonsterKey;

/// Wraps database related to battle
#[derive(Debug)]
pub struct BattleData<'a> {
    pub home_summ: Summoner<'a>,
    pub oppo_summ: Summoner<'a>,
    pub monsters: HashMap<MonsterKey, Monster<'a>>,
    pub home_alive: Vec<MonsterKey>,
    pub oppo_alive: Vec<MonsterKey>,
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
            home_alive,
            oppo_alive,
        }
    }

    pub fn get_team_vecs(home: &Vec<Monster<'a>>, oppo: &Vec<Monster<'a>>) -> (Vec<MonsterKey>, Vec<MonsterKey>) {
        let mut home_vec = Vec::new();
        let mut oppo_vec = Vec::new();
        for mons in home {
            home_vec.push(MonsterKey::Home(mons.get_pos()));
        }
        for mons in oppo {
            oppo_vec.push(MonsterKey::Oppo(mons.get_pos()));
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
            let mk = MonsterKey::Home(mons.get_pos() as u8);
            map.insert(mk, mons);
        }
        for mons in oppo.into_iter() {
            let mk = MonsterKey::Oppo(mons.get_pos() as u8);
            map.insert(mk, mons);
        }
        map
    }

    pub fn get_random_alive_enemy(&self, mk: &MonsterKey) -> Option<&MonsterKey> {
        let mut rng = thread_rng();
        match mk {
            MonsterKey::Home(_) => self.oppo_alive.choose(&mut rng),
            MonsterKey::Oppo(_) => self.home_alive.choose(&mut rng),
        }
    }

    pub fn get(&self, mk: &MonsterKey) -> Option<&Monster<'a>> {
        self.monsters.get(mk)
    }

    pub fn get_mut(&mut self, mk: &MonsterKey) -> Option<&mut Monster<'a>> {
        self.monsters.get_mut(mk)
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
    }

    pub fn deal_true_damage(&mut self, mk: &MonsterKey, dmg: i32) {
        let monster = self.monsters.get_mut(mk).expect("mk is not part of battle");
        let m_health = monster.get_health();
        monster.set_health(m_health - dmg);
    }

    /// Register team-wide buffs
    pub fn register_team_buffs(&mut self, mk: &MonsterKey) {
        
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