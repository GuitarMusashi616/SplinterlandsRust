use std::collections::BinaryHeap;

use crate::monster::Monster;
use crate::summoner::Summoner;
use rand::thread_rng;

pub struct Deck {
    summoner: Summoner,
    monsters: Vec<Monster>,
}

impl Deck {
    pub fn new(summoner: Summoner, monsters: Vec<Monster>) -> Self {
        Self {
            summoner,
            monsters,
        }
    }

    pub fn get_monsters(&self) -> &Vec<Monster> {
        return &self.monsters
    }

    pub fn get_highest_speed(&self) {
        // takes O(N) to get every time, just pop from heap
        // only O(6) to find each time * n rounds
        // each round will pop from heap, if dead will skip
        // self.monsters
        // .iter()
        // .filter(|x| x.is_alive())
        // .fold(vec![0], |acc, x| {
        //     if x.get_speed() > acc[0] {
        //         return x.get_speed();
        //     }
        //     acc
        // });
    }
}