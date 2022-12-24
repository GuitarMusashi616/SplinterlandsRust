use std::collections::BinaryHeap;

use rand::{seq::SliceRandom, thread_rng};

use super::{monsterkey::MonsterKey, battledata::BattleData};


/// Wrapper for a heap of monster references, returns them in order, when tied for speed randomly picks one
pub struct RoundRobin<T: Ord + Clone> {
    tied: Vec<T>,
    heap: BinaryHeap<T>,
}

impl<T: Ord + Clone> RoundRobin<T> {
    /// Fills internal heap with references to monsters in home deck and opponent deck
    pub fn new(elements: Vec<T>) -> Self {
        Self {
            tied: Vec::new(),
            heap: BinaryHeap::from(elements),
        }
    }

    /// All monsters in a round put into a heap for the purporses of deciding which goes first
    // pub fn get_heap(home: &Vec<T>, oppo: &Vec<Rc<RefCell<Monster>>>) -> BinaryHeap<Rc<RefCell<Monster>>> {
    //     let mut all_mons: Vec<Rc<RefCell<Monster>>> = home.iter().map(|x| Rc::clone(x)).collect();
    //     all_mons.extend(oppo.iter().map(|x| Rc::clone(x)));
    //     all_mons.into()
    // }

    /// Decides who goes next
    pub fn pop(&mut self) -> Option<T> {
        // if theres rem in the tied vec, then pop randomly from there
        if !self.tied.is_empty() {
            return self.tied.pop()
        }

        let mut pick = self.heap.pop();
        // while the top of heap has same speed
        // fill the tied vec
        //
        loop {
            match self.heap.peek() {
                Some(cur) => {
                    if let Some(prev) = pick.take() {
                        if prev == *cur {
                            self.tied.push(prev);
                            pick = self.heap.pop();
                            continue;
                        }
                        break;
                    }
                    break;
                },
                None => {break;},
            }
        }

        if !self.tied.is_empty() {
            if let Some(x) = pick.take() {
                self.heap.push(x);
            }
            self.tied.shuffle(&mut thread_rng());
            self.tied.pop()
        } else {
            pick
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tied.is_empty() && self.heap.is_empty()
    }
}