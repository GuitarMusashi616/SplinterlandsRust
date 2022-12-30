use std::{collections::{HashSet, HashMap}, slice::Iter, vec::IntoIter};

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

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

    pub fn len(&self) -> usize {
        self.keys.len()
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

    pub fn least_health(&self, bd: &BattleData) -> Option<MonsterKey> {
        self.keys.iter().fold((i32::MAX, None), |mut acc, elem| {
            let mons = bd.get(elem).unwrap();
            let health = mons.get_health();
            if health < acc.0 {
                acc = (health, Some(*elem));
            }
            acc
        }).1
    }

    pub fn random_from_filter(&self, f: impl Fn(&MonsterKey) -> bool) -> Option<MonsterKey> {
        let filtered: Vec<_> = self.keys.iter().filter(|x| f(x)).collect();
        filtered.choose(&mut thread_rng()).copied().copied()
    }

    pub fn first_from_filter(&self, f: impl FnMut(&&MonsterKey) -> bool) -> Option<MonsterKey> {
        self.keys.iter().find(f).copied()
    }

    pub fn get_taunt(&self, bd: &BattleData) -> Option<MonsterKey> {
        let filtered: Vec<_> = self.keys.iter().filter(|x| {
            let mons = bd.get(x);
            if mons.is_none() {
                return true;
            }
            mons.unwrap().has_ability(Ability::Taunt)
        }).collect();
        filtered.choose(&mut thread_rng()).copied().copied()
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
