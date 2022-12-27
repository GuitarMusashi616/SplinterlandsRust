use std::collections::{BinaryHeap, HashMap, HashSet, LinkedList};
use std::os::windows::thread;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use super::monsterspeed::MonsterSpeed;

// must make K into int or make V into K
pub struct RoundRobinIter {
    heap: BinaryHeap<u8>,
    dict: HashMap<u8, Vec<MonsterSpeed>>,
}

impl RoundRobinIter {
    pub fn new(values: Vec<MonsterSpeed>) -> Self {
        let mut map = Self::make_map(&values);
        Self::shuffle_map(&mut map);
        let mut heap = Self::make_heap(&map);
        // println!("map: {:?}\nheap: {:?}", map, heap);
        Self {
            heap,
            dict: map,
        }
    }

    pub fn make_heap(map: &HashMap<u8, Vec<MonsterSpeed>>) -> BinaryHeap<u8> {
        let mut arr: Vec<u8> = map.keys().map(|x|x.clone()).collect();
        BinaryHeap::from(arr)
    }

    pub fn make_map(values: &Vec<MonsterSpeed>) -> HashMap<u8, Vec<MonsterSpeed>> {
        let mut map = HashMap::new();
        for key in values {
            if !map.contains_key(&key.speed) {
                map.insert(key.speed, Vec::new());
            }
            let cards = map.get_mut(&key.speed).unwrap();
            cards.push(key.clone());
        }
        map
    }

    pub fn shuffle_map(map: &mut HashMap<u8, Vec<MonsterSpeed>>) {
        let mut rng = thread_rng();
        for arr in map.values_mut() {
            arr.shuffle(&mut rng);
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.heap.is_empty();
    }

    pub fn pop(&mut self) -> Option<MonsterSpeed> {
        if self.heap.is_empty() {
            return None;
        }
        let key = self.heap.peek().unwrap();
        if !self.dict.contains_key(key) {
            panic!("heap error, contains {:?} not in {:?}", key, self.dict);
        }
        let arr = self.dict.get_mut(key).unwrap();
        // println!("{}: {:?}", key, arr);
        if arr.is_empty() {
            panic!("heap key corresponds to empty list");
            // return None;
        }

        let res = arr.pop().unwrap();
        // println!("res: {:?}", res);
        if arr.is_empty() {
            self.heap.pop();
        }
        Some(res)
    }
}

impl Iterator for RoundRobinIter {
    type Item = MonsterSpeed;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use crate::battles::monsterkey::MonsterKey;

    use super::*;

    #[test]
    fn test_order() {
        let input = vec![
            MonsterSpeed::new(MonsterKey::Home(0), 7),
            MonsterSpeed::new(MonsterKey::Home(1), 2),
            MonsterSpeed::new(MonsterKey::Home(2), 8),
            MonsterSpeed::new(MonsterKey::Home(3), 8),
            MonsterSpeed::new(MonsterKey::Oppo(0), 8),
            MonsterSpeed::new(MonsterKey::Oppo(1), 2),
            MonsterSpeed::new(MonsterKey::Oppo(2), 3),
            MonsterSpeed::new(MonsterKey::Oppo(3), 4),
        ];

        let rr = RoundRobinIter::new(input);
        let mut res = Vec::new();
        for val in rr {
            res.push(val.speed);
        }
        // while !rr.is_empty() {
        //     let val = rr.pop().unwrap();
            // if val.speed == 8 {
            //     println!("{:?}", val.mk)
            // }
            // res.push(val.speed);
        // }

        let exp = vec![8,8,8,7,4,3,2,2];
        assert_eq!(res, exp);
    }
}