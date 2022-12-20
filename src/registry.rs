use std::collections::HashMap;

use crate::carddata::CardData;
use crate::cardparser;
use std::rc::Rc;

#[derive(Debug)]
pub struct Registry {
    cards: HashMap<String, Rc<CardData>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            cards: HashMap::new(),
        }
    }

    pub fn from(filename: &str) -> Rc<Self> {
        let reg = cardparser::get_registry(filename).expect("file not found");
        Rc::new(reg)
    }

    pub fn add(&mut self, card: CardData) {
        self.cards.insert(card.name.clone(), Rc::new(card));
    }

    pub fn get(&self, search: &str) -> Option<&CardData> {
        self.cards.get(search).map(|x|x.as_ref())
    }

    pub fn get_rc(&self, search: &str) -> Option<&Rc<CardData>> {
        self.cards.get(search)
    }
}

