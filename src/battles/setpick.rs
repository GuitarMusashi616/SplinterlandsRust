use std::{collections::{HashSet, HashMap}, slice::Iter, vec::IntoIter};

use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::cardparse::enums::Ability;

use super::{monsterkey::MonsterKey, battledata::BattleData};

/// Pick from a set like it's a vec but with the performance of a set
/// Must keep track of positions like a vec
#[derive(Debug)]
pub struct SetPick {
    keys: Vec<MonsterKey>,
    map: HashMap<MonsterKey, usize>,
}

impl SetPick {
    pub fn new(mks: &[MonsterKey]) -> Self {
        let mut keys = Vec::new();
        let mut map = HashMap::new();
        for i in 0..mks.len() {
            keys.push(mks[i]);
            map.insert(mks[i], i);
        }
        Self {
            keys,
            map
        }
    }

    pub fn contains(&self, mk: &MonsterKey) -> bool {
        if self.map.contains_key(mk) {
            return true;
        }
        false
    }

    pub fn remove(&mut self, mk: &MonsterKey) {
        if !self.map.contains_key(mk) {
            return
        }

        let &index = self.map.get(mk).unwrap();
        self.keys.remove(index);
        self.map.remove(mk);
        for i in index..self.keys.len() {
            let new_mk = self.keys[i];
            self.map.insert(new_mk, i);
        }
    }

    pub fn choose(&self, rng: &mut ThreadRng) -> Option<&MonsterKey> {
        self.keys.choose(rng)
    }

    pub fn get_pos(&self, mk: &MonsterKey) -> Option<&usize> {
        self.map.get(mk)
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn index(&self, i: usize) -> MonsterKey {
        self.keys[i]
    }

    pub fn iter(&self) -> impl Iterator<Item = &MonsterKey> {
        self.keys.iter()
    }

    pub fn to_monster_string(&self, bd: &BattleData) -> String {
        let mons: Vec<_> = self.keys.iter().map(|x| bd.get(x).unwrap()).collect();
        let strings: Vec<_> = mons.iter().map(|x| {
            let mut string = format!("{} ({}/{})",
                x.get_name(),
                x.get_health(),
                x.get_max_health(),
            );
            
            if x.get_armor() > 0 {
                string.push_str(&format!(" [{}]", x.get_armor()));
            }
            string
        }).collect();
        let mut res = strings.join(", ");
        res.insert_str(0, "[ ");
        res.push_str(" ]");
        res
    }

}

#[cfg(test)]
mod tests {
    use crate::{battles::monsterkey::MonsterKey, gamedata::monster::Monster};

    use super::SetPick;

    #[test]
    pub fn test_pop_from_center() {
        let arr = (1..4).map(|x| MonsterKey::Home(x)).collect::<Vec<_>>();
        let mut alive = SetPick::new(&arr);
        let key = MonsterKey::Home(2);
        // println!("{:?}", alive);
        let exp = vec![MonsterKey::Home(1), MonsterKey::Home(3)];
        alive.remove(&key);
        assert_eq!(alive.keys, exp);
        // println!("{:?}", alive);
        let key = MonsterKey::Home(3);
        let exp = vec![MonsterKey::Home(1)];
        alive.remove(&key);
        assert_eq!(alive.keys, exp);
        // println!("{:?}", alive);
    }

}
