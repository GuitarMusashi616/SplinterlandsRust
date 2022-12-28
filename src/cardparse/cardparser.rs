use csv::{Reader, DeserializeRecordsIter};
use serde::Deserialize;
use std::{fs, collections::HashMap};

use crate::gamedata::registry::Registry;

use super::carddata::CardData;

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

/// Registers Summoners and Monsters
pub fn get_map(path: &str) -> Result<HashMap<String, CardData>, csv::Error> {
    let mut map = HashMap::new();
    let content = fs::read_to_string(path)?;
    let mut reader = Reader::from_reader(content.as_bytes());
    for data in reader.deserialize() {
        let raw_card: RawCardData = data?;
        let card: CardData = raw_card.into();
        map.insert(card.name.to_owned(), card);
    }

    Ok(map)
}
