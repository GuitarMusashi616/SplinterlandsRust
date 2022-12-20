use rand::{seq::SliceRandom, thread_rng};

use crate::monster::Monster;


pub fn target_for(hero: &Monster, enemies: &Vec<Monster>) {
    
}

pub fn target_random(enemies: &Vec<Monster>) -> Option<&Monster> {
    let mut rng = thread_rng();
    enemies.choose(&mut rng)
}