use std::collections::BinaryHeap;
use rand::{thread_rng, rngs::ThreadRng, seq::SliceRandom};

use crate::monster::Monster;

/// wrapper for a heap of monster references, returns them in order, when tied for speed randomly picks one
pub struct RoundRobin<'a> {
    tied: Vec<&'a Monster>,
    heap: BinaryHeap<&'a Monster>,
    rng: ThreadRng,
}

impl<'a> RoundRobin<'a> {
    /// Fills internal heap with references to monsters in home deck and opponent deck
    pub fn new(home: &'a Vec<Monster>, oppo: &'a Vec<Monster>) -> Self {
        Self {
            tied: Vec::new(),
            heap: Self::get_heap(home, oppo),
            rng: thread_rng(),
        }
    }

    /// All monsters in a round put into a heap for the purporses of deciding which goes first
    pub fn get_heap(home: &'a Vec<Monster>, oppo: &'a Vec<Monster>) -> BinaryHeap<&'a Monster> {
        let mut all_mons: Vec<&Monster> = home.iter().collect();
        all_mons.extend(oppo);
        all_mons.into()
    }

    /// Decides who goes next
    pub fn pop(&mut self) -> Option<&Monster> {
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
                Some(&cur) => {
                    if let Some(prev) = pick {
                        if prev.get_speed() == cur.get_speed() {
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
            if let Some(val) = pick {
                self.tied.push(val);
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