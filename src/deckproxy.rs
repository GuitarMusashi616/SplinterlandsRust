use crate::{registry::{Registry}, enums::Role};
use crate::deck::Deck;
use crate::summoner::Summoner;
use crate::monster::Monster;

pub struct DeckProxy<'a> {
    registry: &'a Registry,
    cards: &'a[&'a str],
}

impl<'a> DeckProxy<'a> {
    pub fn new(registry: &'a Registry, cards: &'a[&'a str]) -> Self {
        Self {
            registry,
            cards,
        }
    }

    pub fn instantiate(self) -> Deck {
        let mut summoner = None;
        let mut monsters = Vec::new();
        for &card_name in self.cards {
            let card_data = self.registry.get_rc(card_name).unwrap();
            match card_data.role {
                Role::Monster => {monsters.push(Monster::new(card_data));},
                Role::Summoner => {summoner = Some(Summoner::new(card_data));},
            }
        }
        Deck::new(summoner.expect("no summoner in deck proxy"), monsters)
    }
}