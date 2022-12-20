use std::collections::HashSet;

use crate::cardparser::RawCardData;
use crate::enums::*;

/// raw card data cleaned up eg. with enums instead of strings
#[derive(Debug)]
pub struct CardData {
    pub name: String,
    pub role: Role,
    pub element: Element,
    pub mana_cost: i32,
    pub damage: i32,
    pub attack_type: AttackType,
    pub speed: i32,
    pub health: i32,
    pub armor: i32,
    pub abilities: HashSet<Ability>,
}

impl From<RawCardData> for CardData {
    fn from(raw_card_data: RawCardData) -> Self {
        Self {
            name: raw_card_data.Card,
            role: raw_card_data.Role.as_str().into(),
            element: raw_card_data.Element.as_str().into(),
            mana_cost: raw_card_data.ManaCost,
            damage: raw_card_data.Dmg,
            attack_type: raw_card_data.AttackType.as_str().into(),
            speed: raw_card_data.Speed,
            health: raw_card_data.Health,
            armor: raw_card_data.Armor,
            abilities: Ability::make_set(&[&raw_card_data.Ability1, &raw_card_data.Ability2]),
        }
    }
}