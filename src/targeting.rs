use rand::{seq::SliceRandom, thread_rng};

use crate::monster::Monster;


pub fn target_for(hero: &Monster, enemies: &Vec<Monster>) {
    
}

pub fn target_random<T>(enemies: &Vec<T>) -> Option<&T> {
    let mut rng = thread_rng();
    enemies.choose(&mut rng)
}