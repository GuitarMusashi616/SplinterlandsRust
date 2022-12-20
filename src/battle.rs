use std::collections::BinaryHeap;

use crate::cardparser;
use crate::registry::Registry;
use crate::battlefactory::BattleFactory;
use crate::enums::Outcome;
use crate::deck::Deck;
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
    pub fn get_monster_heap(&self) -> BinaryHeap<&Monster> {
        let mut all_mons: Vec<&Monster> = self.home.get_monsters().iter().collect();
        all_mons.extend(self.oppo.get_monsters());
        all_mons.into()
    }

    /// every alive monster gets a turn to punch another monster
    pub fn round(&mut self) {
        let mut round_robin = RoundRobin::new(self.home.get_monsters(), self.oppo.get_monsters());
        // let mut round_robin = self.get_monster_heap();

        while !round_robin.is_empty() {
            let pick = round_robin.pop();
            if let Some(x) = pick {
                println!("{}", x.get_name())
            }
        }
        println!("");
        // key = all_mons.pop()
        // while key.is_dead()
        //     key = all_mons.pop()
        // once all_mons is empty... all mons have had a turn
    }

    pub fn battle(&mut self) -> Outcome {
        // battle is in Battle class which has instanced decks of instanced monsters
        // round function
        // pick next monster function

        // let mons1 = self.home.get_highest_speed();  // if tied pick randomly
        // let mons2 = self.deck.get_highest_speed();
        // let mons = Random::choice(mons1, mons2);

        // pick target (use strategy)
        // upon instantiation, monster will get reference to function corresponding to target strategy

        // let enemy = mons.target(&self);

        // attack enemy with monster
        // might need to inject more context later in case it matters to some cards

        // mons.attack(enemy);
        
        Outcome::Win
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
        
        // let deck1 = Deck::new(summoner1, &[monster1]);
        // let deck2 = Deck::new(summoner2, &[monster2, monster3]);

        // let battle = Battle::new(deck1, deck2);
        // methods that compose battle.round()

        // let mons = battle.next_monster();
        // assert_eq!(mons.name, "Spineback Wolf");
        // let oppo = battle.cur_monster_target();
        // assert_eq!(oppo.name, "Death Elemental");
        // battle.cur_monster_attack();
        // make it lose health
        // continue until no monsters are alive
        // have other tests test buffs
        // then methods to make all combinations of decks possible
        // then tournament to rank the deck combinations and voila

    

    }

    #[test]
    fn test_better_battle() {
        let registry = Registry::from("assets/cards.csv");
        // strings to be taken from registry therefore static
        // let deck1 = DeckProxy::new("Drake of Arnak", "Spineback Wolf");
        // let deck2 = DeckProxy::new("Contessa L'ament", "Death Elemental");

        // let summoner = CardProxy::new("Drake of Arnak");
        // let monster = CardProxy::new("Spineback Wolf");

        // let deckInst = deck1.instantiate(registry);
        // let summInst = summoner.instantiate(registry);
        // let monsInst = monster.instantiate(registry);

        let bf = BattleFactory::new(&registry);
        let mut battle_proxy = bf.create(&["Drake of Arnak", "Spineback Wolf"], &["Contessa L'ament", "Death Elemental"]);
        let mut battle = battle_proxy.instansiate();
        let mut stuff = 4;
        // tells you if it is valid
        // BattleFactory::from("assets/cards.csv")
        let outcome = battle.battle();

        // let battle = battle_proxy.instantiate();
        dbg!(&bf);
        // dbg!(&battle_proxy);

    }

    #[test]
    fn test_heap_bool() {
        let mut heap = BinaryHeap::new();  
        heap.push(5);
    }
}