use std::collections::HashMap;

use crate::gamedata::{monster::Monster, summoner::Summoner, registry::Registry};

use super::monsterkey::MonsterKey;

/// Wraps database related to battle
#[derive(Debug)]
pub struct BattleData<'a> {
    pub home_summ: Summoner<'a>,
    pub oppo_summ: Summoner<'a>,
    pub monsters: HashMap<MonsterKey, Monster<'a>>,
}

impl<'a> BattleData<'a> {
    pub fn new(reg: &'a Registry, home: Vec<&'a str>, oppo: Vec<&'a str>) -> Self {
        let (home_summ, home) = Self::vec_to_deck(reg, home);
        let (oppo_summ, oppo) = Self::vec_to_deck(reg, oppo);
        let monsters = Self::to_map(home, oppo);
        Self {
            home_summ,
            oppo_summ,
            monsters
        }
    }

    pub fn vec_to_deck(reg: &'a Registry, cards: Vec<&'a str>) -> (Summoner<'a>, Vec<Monster<'a>>) {
        let mut monsters = Vec::new();
        for &key in cards.iter().skip(1) {
            if let Some(carddata) = reg.map.get(key) {
                let monster = Monster::new(carddata);
                monsters.push(monster);
            }
        }
        let summ = reg.map.get(cards[0]).expect("1st card must be a summoner");
        (Summoner::new(summ), monsters)
    }

    pub fn to_map(home: Vec<Monster<'a>>, oppo: Vec<Monster<'a>>) -> HashMap<MonsterKey, Monster<'a>> {
        let mut map = HashMap::new();
        for (i, mons) in home.into_iter().enumerate() {
            let mk = MonsterKey::Home(i as u8);
            map.insert(mk, mons);
        }
        for (i, mons) in oppo.into_iter().enumerate() {
            let mk = MonsterKey::Oppo(i as u8);
            map.insert(mk, mons);
        }
        map
    }
}