#![allow(non_snake_case)]

use gamedata::registry::Registry;

use crate::battles::battle::Battle;


mod gamedata;
mod battles;
mod cardparse;

fn main() {
    let registry = Registry::from("assets/cards.csv");
    let home = vec!["Drake of Arnak", "Goblin Shaman"];
    let oppo = vec!["Pyre", "Spineback Turtle"];

    let battle = Battle::new(&registry, home, oppo);

    dbg!(battle);
}
