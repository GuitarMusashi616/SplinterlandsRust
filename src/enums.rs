use std::{panic, collections::HashSet};

#[derive(Debug)]
pub enum Role {
    Summoner,
    Monster,
}

impl From<&str> for Role {
    fn from(string: &str) -> Self {
        match string {
            "summoner" => Self::Summoner,
            "monster" => Self::Monster,
            &_ => {panic!("{} could not be cast into Role", string);},
        }
    }
}

#[derive(Debug)]
pub enum Element {
    Fire,
    Water,
    Neutral,
    Earth,
    Life,
    Death,
    Dragon,
}

impl From<&str> for Element {
    fn from(string: &str) -> Self {
        match string {
            "fire" => Self::Fire,
            "water" => Self::Water,
            "neutral" => Self::Neutral,
            "earth" => Self::Earth,
            "life" => Self::Life,
            "death" => Self::Death,
            "dragon" => Self::Dragon,
            &_ => {panic!("{} could not be cast into Element", string);},
        }
    }
}

#[derive(Debug)]
pub enum AttackType {
    None,
    Melee,
    Ranged,
    Magic,
}

impl From<&str> for AttackType {
    fn from(string: &str) -> Self {
        match string {
            "none" => Self::None,
            "melee" => Self::Melee,
            "ranged" => Self::Ranged,
            "magic" => Self::Magic,
            &_ => {panic!("{} could not be cast into AttackType", string);},
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Ability {
    Melee(i8),
    Ranged(i8),
    Magic(i8),
    Speed(i8),
    Armor(i8),
    Health(i8),
    MagicReflect,
    Shield,
    Flying,
    Heal,
    Void,
    Dodge,
    Reach,
    Stun,
    Sneak,
    Blast,
    Weaken,
    Inspire,
    Snipe,
    Opportunity,
    Protect,
    TankHeal,
    Taunt,
    Retaliate,
}

impl From<&str> for Ability {
    fn from(string: &str) -> Self {
        match string {
            "melee" => Self::Melee(0),
            "melee+" => Self::Melee(1),
            "melee-" => Self::Melee(-1),
            "ranged" => Self::Ranged(0),
            "ranged-" => Self::Ranged(-1),
            "magic" => Self::Magic(0),
            "magic+" => Self::Magic(1),
            "magic-" => Self::Magic(-1),
            "speed" => Self::Speed(0),
            "speed+" => Self::Speed(1),
            "armor" => Self::Armor(0),
            "armor+" => Self::Armor(1),
            "armor--" => Self::Armor(-2),
            "health" => Self::Health(0),
            "health+" => Self::Health(1),
            "magic reflect" => Self::MagicReflect,
            "shield" => Self::Shield,
            "flying" => Self::Flying,
            "heal" => Self::Heal,
            "void" => Self::Void,
            "dodge" => Self::Dodge,
            "reach" => Self::Reach,
            "stun" => Self::Stun,
            "sneak" => Self::Sneak,
            "blast" => Self::Blast,
            "weaken" => Self::Weaken,
            "inspire" => Self::Inspire,
            "snipe" => Self::Snipe,
            "opportunity" => Self::Opportunity,
            "protect" => Self::Protect,
            "tank heal" => Self::TankHeal,
            "taunt" => Self::Taunt,
            "retaliate" => Self::Retaliate,
            &_  => {panic!("{} could not be cast into Ability", string);},
        }
    }
}

impl Ability {
    pub fn make_set(strings: &[&str]) -> HashSet<Ability> {
        let mut set = HashSet::new();
        for &string in strings {
            if string.is_empty() {
                continue;
            }
            set.insert(string.into());
        }
        set
    }
}

#[derive(Debug)]
pub enum Outcome {
    Win,
    Draw,
    Lose,
}