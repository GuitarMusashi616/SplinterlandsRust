use std::collections::{HashMap, HashSet};

use crate::gamedata::{monster::Monster, summoner::Summoner, registry::Registry};

use super::monsterkey::MonsterKey;

/// Wraps database related to battle
#[derive(Debug)]
pub struct BattleData<'a> {
    pub home_summ: Summoner<'a>,
    pub oppo_summ: Summoner<'a>,
    pub monsters: HashMap<MonsterKey, Monster<'a>>,
    pub home_alive: HashSet<MonsterKey>,
    pub oppo_alive: HashSet<MonsterKey>,
}

impl<'a> BattleData<'a> {
    pub fn new(reg: &'a Registry, home: Vec<&'a str>, oppo: Vec<&'a str>) -> Self {
        let (home_summ, home) = Self::vec_to_deck(reg, home);
        let (oppo_summ, oppo) = Self::vec_to_deck(reg, oppo);
        let monsters = Self::to_map(home, oppo);
        let (home_alive, oppo_alive) = Self::get_team_sets(&monsters);
        Self {
            home_summ,
            oppo_summ,
            monsters,
            home_alive,
            oppo_alive,
        }
    }

    pub fn get_team_sets(monsters: &HashMap<MonsterKey, Monster>) -> (HashSet<MonsterKey>, HashSet<MonsterKey>) {
        let mut home_set = HashSet::new();
        let mut oppo_set = HashSet::new();
        for mk in monsters.keys() {
            match mk {
                MonsterKey::Home(_) => home_set.insert(*mk),
                MonsterKey::Oppo(_) => oppo_set.insert(*mk),
            };
        }
        (home_set, oppo_set)
    }

    pub fn vec_to_deck(reg: &'a Registry, cards: Vec<&'a str>) -> (Summoner<'a>, Vec<Monster<'a>>) {
        let mut monsters = Vec::new();
        for (i, &key) in cards.iter().skip(1).enumerate() {
            if let Some(carddata) = reg.map.get(key) {
                let monster = Monster::new(carddata, i as i32);
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

    // pub fn get_random_rem_enemy(mk: MonsterKey) -> Option<&HashSet<MonsterKey>> {
    //     if 
    // }

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