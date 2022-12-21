use std::{collections::BinaryHeap, cell::RefCell, rc::Rc};
use rand::{thread_rng, rngs::ThreadRng, seq::SliceRandom};

use crate::monster::Monster;

/// Wrapper for a heap of monster references, returns them in order, when tied for speed randomly picks one
pub struct RoundRobin {
    tied: Vec<Rc<RefCell<Monster>>>,
    heap: BinaryHeap<Rc<RefCell<Monster>>>,
    rng: ThreadRng,
}

impl RoundRobin {
    /// Fills internal heap with references to monsters in home deck and opponent deck
    pub fn new(home: &Vec<Rc<RefCell<Monster>>>, oppo: &Vec<Rc<RefCell<Monster>>>) -> Self {
        Self {
            tied: Vec::new(),
            heap: Self::get_heap(home, oppo),
            rng: thread_rng(),
        }
    }

    /// All monsters in a round put into a heap for the purporses of deciding which goes first
    pub fn get_heap(home: &Vec<Rc<RefCell<Monster>>>, oppo: &Vec<Rc<RefCell<Monster>>>) -> BinaryHeap<Rc<RefCell<Monster>>> {
        let mut all_mons: Vec<Rc<RefCell<Monster>>> = home.iter().map(|x| Rc::clone(x)).collect();
        all_mons.extend(oppo.iter().map(|x| Rc::clone(x)));
        all_mons.into()
    }

    /// Decides who goes next
    pub fn pop(&mut self) -> Option<Rc<RefCell<Monster>>> {
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
                    if let Some(ref prev) = pick {
                        if prev.borrow().get_speed() == cur.borrow().get_speed() {
                            self.tied.push(Rc::clone(prev));
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
            if let Some(x) = pick {
                self.heap.push(x);
            }
            self.tied.shuffle(&mut self.rng);
            self.tied.pop()
        } else {
            pick
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tied.is_empty() && self.heap.is_empty()
    }
}