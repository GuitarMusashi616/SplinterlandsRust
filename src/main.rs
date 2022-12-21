#![allow(non_snake_case)]
#![allow(dead_code)]

use battlefactory::BattleFactory;
use battleproxy::BattleProxy;
use registry::Registry;

mod cardparser;
mod monster;
mod summoner;
mod registry;
mod battle;
mod deck;
mod deckproxy;
mod enums;
mod battlefactory;
mod battleproxy;
mod carddata;
mod roundrobin;
mod targeting;

fn test_reg() {
    let registry = cardparser::get_registry("assets/cards.csv").unwrap();

    let summoner1 = registry.get("Drake of Arnak");
    let summoner2 = registry.get("Contessa L'ament");
    let monster1 = registry.get("Spineback Wolf");
    let monster2 = registry.get("Bone Golem");
    let monster3 = registry.get("Death Elemental");

    
    println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}", summoner1, summoner2, monster1, monster2, monster3);

    // what happens if create role but doesn't exist?
    let good_role = enums::Role::from("summoner");
    dbg!(good_role);
}

fn print_parse() {
    let _ = cardparser::dbg_parse();
}

fn fixture1() {
    let registry = Registry::from("assets/cards.csv");
    let bf = BattleFactory::new(&registry);

    let a = [
        "Alric Stormbringer",
        "Elven Defender",
        "Serpent of Eld",
        "Ice Pixie",
        "Enchanted Pixie",
        "Pirate Captain",
        "Medusa"
    ];
    let b = [
        "Lyanna Natura",
        "Unicorn Mustang",
        "Stone Golem",
        "Goblin Thief",
        "Goblin Sorcerer",
        "Child of the Forest",
        "Centaur",
    ];

    let mut bp = bf.create(&a, &b);
    let mut battle = bp.instansiate();
    battle.battle();
}

fn battle() {
    let registry = Registry::from("assets/cards.csv");
    let bf = BattleFactory::new(&registry);
    let mut battle_proxy = bf.create(&["Drake of Arnak", "Spineback Wolf", "Spark Pixies"], &["Contessa L'ament", "Death Elemental", "Child of the Forest"]);
    let mut battle = battle_proxy.instansiate();
    battle.battle();
}

fn main() {
    // test_reg();
    // battle();   
    fixture1();
}
