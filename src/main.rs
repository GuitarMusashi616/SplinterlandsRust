#![allow(non_snake_case)]

use std::collections::{HashSet, HashMap};

use cardparse::enums::Element;
use gamedata::registry::Registry;
use tourney::combos::{tournament, super_tournament};

use crate::{battles::battle::Battle, cardparse::enums::Ability};
use clappers::Clappers;


mod gamedata;
mod battles;
mod cardparse;
mod tourney;

pub fn unique_abilities() {
    let registry = Registry::from("assets/new_cards.csv");
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

    let mut battle = Battle::new(&registry, &home, &oppo);
    battle.game();
}

pub fn example_battle_2() {
    let reg = Registry::from("assets/new_cards.csv");
    let home = vec!["Tarsa", "Living Lava", "Magma Troll", "Tenyii Striker", "Serpentine Spy", "Lava Spider"];
    let oppo = vec!["Kelya Frendul", "Serpent of Eld", "Feasting Seaweed", "Sniping Narwhal", "Ice Pixie"];
    let mut battle = Battle::new(&reg, &home, &oppo);
    battle.game();
}

fn main() {
    // example_battle_2();
    // unique_abilities();

    // set defaults
    let mut element = Element::Fire;
    let mut mana = 16;
    let mut train = 10;
    let mut lines = 50;

    let clappers = Clappers::build()
        .set_flags(vec![
            "h|help",
        ])
        .set_singles(vec![
            "e|element",
            "m|mana",
            "n|lines",
            "t|train",
        ])
        .parse();

    if clappers.get_flag("help") {
        println!("
            usage: splint [arguments]
            eg:    splint -e fire -m 20

            Arguments:
                -h|--help                        Print this help
                -e|--element element             Select element
                -m|--mana max_mana               Select max mana
                -n|--lines                       # of lines of output
                -t|--train                       # of battles to determine Elo
        ");
    }

    let element_str = clappers.get_single("element");
    if !element_str.is_empty() {
        element = match element_str.as_ref() {
            "fire" => Element::Fire,
            "water" => Element::Water,
            "earth" => Element::Earth,
            "life" => Element::Life,
            "death" => Element::Death,
            "all" => Element::Neutral,
            _ => panic!("{} is not a recognized/supported element", element_str),
        }
    }

    mana = clappers.get_single("mana").parse().unwrap_or(mana);
    train = clappers.get_single("train").parse().unwrap_or(train);
    lines = clappers.get_single("lines").parse().unwrap_or(lines);

    println!("Element: {}\nMana: {}\nTrain: {}\nLines: {}", element_str, mana, train, lines);

    let reg = Registry::from("assets/new_cards.csv");
    if element == Element::Neutral {
        super_tournament(&reg, mana, train, lines);
        return;
    }
    tournament(&reg, element, mana, train, lines);

    // let reg = Registry::from("assets/new_cards.csv");
    // tournament(&reg, element, mana, train, lines);
}
