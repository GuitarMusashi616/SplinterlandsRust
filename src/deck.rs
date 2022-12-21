use std::cell::RefCell;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Display;
use std::rc::Rc;

use crate::monster::Monster;
use crate::summoner::Summoner;
use rand::thread_rng;

#[derive(Debug)]
pub struct Deck {
    summoner: Summoner,
    monsters: Vec<Rc<RefCell<Monster>>>,
}

impl Deck {
    pub fn new(summoner: Summoner, monsters: Vec<Rc<RefCell<Monster>>>) -> Self {
        Self {
            summoner,
            monsters,
        }
    }

    pub fn get_monsters(&self) -> &Vec<Rc<RefCell<Monster>>> {
        return &self.monsters
    }

    pub fn is_member(&self, monster_cell: &Rc<RefCell<Monster>>) -> bool {
        for monster in &self.monsters {
            if monster.borrow().get_name() == monster_cell.borrow().get_name() {
                return true
            }
        }
        return false
    }

    pub fn still_standing(&self) -> bool {
        for monster in &self.monsters {
            if monster.borrow().get_health() > 0 {
                return true
            }
        }
        return false
    }

}

impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut status = Vec::new();
        for mons in &self.monsters {
            let borrow = mons.borrow();
            let name = borrow.get_name();
            let health = borrow.get_health();
            let max_health = borrow.get_max_health();
            let content = format!("{}({}/{})", name, health, max_health);
            status.push(content);
        }
        write!(f, "{:?}", status);
        Ok(())
    }
}