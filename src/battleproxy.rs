use crate::{registry::Registry, battle::Battle, deck::Deck, deckproxy::DeckProxy};
use std::rc::Rc;

#[derive(Debug)]
pub struct BattleProxy<'a> {
    registry: Rc<Registry>,
    home: &'a[&'a str],
    oppo: &'a[&'a str],
}

impl<'a> BattleProxy<'a> {
    pub fn new(registry: &Rc<Registry>, home: &'a[&'a str], oppo: &'a[&'a str]) -> Self {
        Self {
            registry: Rc::clone(registry),
            home,
            oppo
        }
    }

    pub fn instansiate(self) -> Battle {
        let deck1 = DeckProxy::new(&self.registry, self.home);
        let deck2 = DeckProxy::new(&self.registry, self.oppo);

        let a = deck1.instantiate();
        let b = deck2.instantiate();

        Battle::new(a, b)
    }

}