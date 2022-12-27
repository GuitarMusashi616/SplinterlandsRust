#![allow(non_snake_case)]

use std::collections::HashSet;

use gamedata::registry::Registry;

use crate::battles::battle::Battle;


mod gamedata;
mod battles;
mod cardparse;

pub fn unique_abilities() {
    let registry = Registry::from("assets/cards.csv");
    let mut all_abilities = HashSet::new();
    for card in registry.map.values() {
        for ability in &card.abilities {
            all_abilities.insert(ability);
        }
    }
    println!("{:?}", all_abilities);
}

pub fn example_battle() {
    let registry = Registry::from("assets/cards.csv");

    let home = vec!["Drake of Arnak", "Goblin Shaman", "Fire Beetle"];
    let oppo = vec!["Pyre", "Spineback Turtle", "Kobold Bruiser"];

    let mut battle = Battle::new(&registry, home, oppo);
    battle.game();
}
fn main() {
    example_battle();
}
