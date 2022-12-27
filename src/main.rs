#![allow(non_snake_case)]

use std::collections::{HashSet, HashMap};

use gamedata::registry::Registry;

use crate::{battles::battle::Battle, cardparse::enums::Ability};


mod gamedata;
mod battles;
mod cardparse;

pub fn unique_abilities() {
    let registry = Registry::from("assets/cards.csv");
    let mut all_abilities: HashMap<Ability, i32> = HashMap::new();
    for card in registry.map.values() {
        for ability in &card.abilities {
            let ability_count = all_abilities.get(ability).copied().unwrap_or_default();
            all_abilities.insert(*ability, ability_count + 1);
        }
    }
    let mut as_vec: Vec<_> = all_abilities.into_iter().collect();
    as_vec.sort_by(|a, b| b.1.cmp(&a.1));
    // println!("{:?}", all_abilities);
    as_vec.into_iter().for_each(|(ability, count)| {
        println!("{:?}: {}", ability, count);
    });
}

pub fn example_battle() {
    let registry = Registry::from("assets/cards.csv");

    let home = vec!["Drake of Arnak", "Goblin Shaman", "Fire Beetle"];
    let oppo = vec!["Pyre", "Spineback Turtle", "Kobold Bruiser"];

    let mut battle = Battle::new(&registry, home, oppo);
    battle.game();
}
fn main() {
    // example_battle();
    unique_abilities();
}
