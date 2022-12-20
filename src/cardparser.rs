use csv::{Reader, DeserializeRecordsIter};
use serde::Deserialize;
use std::{fs, collections::HashSet, io};

use crate::{registry::Registry, carddata::CardData};

/// Raw parsed data for each card
#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct RawCardData {
    pub Card: String,
    pub Role: String,
    pub Element: String,
    pub ManaCost: i32,
    pub Dmg: i32,
    pub AttackType: String,
    pub Speed: i32,
    pub Health: i32,
    pub Armor: i32,
    pub Ability1: String,
    pub Ability2: String,
}

/// Prints out all parsed CardDatas
pub fn dbg_parse() -> Result<(), csv::Error> {
    let content = fs::read_to_string("assets/cards.csv")?;
    // println!("{}", content);
    let mut reader = Reader::from_reader(content.as_bytes());
    for data in reader.deserialize() {
        let raw_card: RawCardData = data?;
        let card: CardData = raw_card.into();
        dbg!(card);
    }
    Ok(())
}

/// Prints out all unique abilities
pub fn abilities_in_csv(path: &str) -> Result<(), csv::Error> {
    let content = fs::read_to_string(path)?;
    let mut hashset = HashSet::new();
    // println!("{}", content);
    let mut reader = Reader::from_reader(content.as_bytes());
    for data in reader.deserialize() {
        let card: RawCardData = data?;
        hashset.insert(card.Ability1.clone());
        hashset.insert(card.Ability2.clone());
        // dbg!(card);
    }

    dbg!(hashset);
    Ok(())
}

/// Registers Summoners and Monsters
pub fn get_registry(path: &str) -> Result<Registry, csv::Error> {
    let mut registry = Registry::new();
    let content = fs::read_to_string(path)?;
    let mut reader = Reader::from_reader(content.as_bytes());
    for data in reader.deserialize() {
        let raw_card: RawCardData = data?;
        let card: CardData = raw_card.into();
        registry.add(card);
    }

    Ok(registry)
}

// Instantiates carddata into summoner/monster and adds it to registry
// pub fn from_card(card: &CardData, registry: &mut SummonerRegistry, monster: &mut MonsterRegistry) {

// }