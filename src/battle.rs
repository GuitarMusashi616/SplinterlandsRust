use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::fmt::Display;
use std::rc::Rc;

use crate::{cardparser, targeting};
use crate::registry::Registry;
use crate::battlefactory::BattleFactory;
use crate::enums::Outcome;
use crate::deck::Deck;
use rand::seq::SliceRandom;
use rand::{thread_rng, rngs::ThreadRng};
use crate::monster::Monster;
use crate::roundrobin::RoundRobin;
pub struct Battle {
    home: Deck,
    oppo: Deck,
}

impl Battle {
    pub fn new(home: Deck, oppo: Deck) -> Self {
        Self {
            home,
            oppo,
        }
    }

    /// all monsters in a round put into a heap for the purporses of deciding which goes first
    pub fn get_monster_heap(&self) -> BinaryHeap<Rc<RefCell<Monster>>> {
        let mut all_mons: Vec<Rc<RefCell<Monster>>> = self.home.get_monsters().iter().map(|x| Rc::clone(x)).collect();
        all_mons.extend(self.oppo.get_monsters().iter().map(|x| Rc::clone(x)));
        all_mons.into()
    }

    /// every alive monster gets a turn to punch another monster
    pub fn round(&mut self) {
        let mut round_robin = RoundRobin::new(self.home.get_monsters(), self.oppo.get_monsters());
        // let mut round_robin = self.get_monster_heap();

        while !round_robin.is_empty() {
            let mut poss_pick = round_robin.pop();
            if poss_pick.is_none() {
                continue;
            }
            let pick = poss_pick.unwrap();
            self.turn(&pick);
        }
        println!("");
    }

    pub fn get_enemies(&self, monster_cell: &Rc<RefCell<Monster>>) -> &Deck {
        if self.home.is_member(monster_cell) {
            return &self.oppo
        }

        if self.oppo.is_member(monster_cell) {
            return &self.home
        }

        panic!("{:?} not part of this home: {:?} or oppo: {:?}", monster_cell.borrow(), self.home, self.oppo)
    }

    pub fn choose_enemy(&self, monster_cell: &Rc<RefCell<Monster>>) -> Option<&Rc<RefCell<Monster>>> {
        let enemies = self.get_enemies(monster_cell);
        let mut rng = thread_rng();
        let mut choice = enemies.get_monsters().choose(&mut rng);
        while let Some(x) = choice {
            if *x != *monster_cell {
                break;
            }
            choice = enemies.get_monsters().choose(&mut rng);
        }
        choice
    }

    pub fn attack(&self, monster: &Rc<RefCell<Monster>>, target: &Rc<RefCell<Monster>>) {
        let monster = monster.borrow();
        let mut target = target.borrow_mut();
        let damage = monster.get_damage();
        let health = target.get_health();
        target.set_health(health - damage);
        println!("{} attacks {} and deals {} damage leaving it with ({}/{}) health", monster.get_name(), target.get_name(), monster.get_damage(), target.get_health(), target.get_max_health());
    }
    
    /// Represents the turn of monster
    pub fn turn(&mut self, monster_cell: &Rc<RefCell<Monster>>) {
        let choice = self.choose_enemy(monster_cell);
        if choice.is_none() {
            return
        }
        let choice = choice.unwrap();

        self.attack(monster_cell, choice);
    }

    pub fn battle(&mut self) -> Outcome {
        let mut i = 1;
        while self.home.still_standing() && self.oppo.still_standing() {
            println!("Round {}:\n\tHome: {}\n\tOppo: {}", i, self.home, self.oppo);

            self.round();
            i += 1;
        }
        if self.home.still_standing() {
            return Outcome::Win
        }
        else if self.oppo.still_standing() {
            return Outcome::Lose
        }
        Outcome::Draw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battle_setup() {
        let registry = cardparser::get_registry("assets/cards.csv").unwrap();

        let summoner1 = registry.get("Drake of Arnak").unwrap();
        let summoner2 = registry.get("Contessa L'ament").unwrap();
        let monster1 = registry.get("Spineback Wolf").unwrap();
        let monster2 = registry.get("Bone Golem").unwrap();
        let monster3 = registry.get("Death Elemental").unwrap();
        
        println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}", summoner1, summoner2, monster1, monster2, monster3);
    }

    #[test]
    fn test_better_battle() {
        // test the targeting
        let reg = Registry::from("assets/cards.csv");
        let bf = BattleFactory::new(&reg);
        let mut battle_proxy = bf.create(&["Drake of Arnak", "Spineback Wolf", "Spark Pixies"], &["Contessa L'ament", "Death Elemental", "Child of the Forest"]);
        let mut battle = battle_proxy.instansiate();
        battle.round();
    }
}