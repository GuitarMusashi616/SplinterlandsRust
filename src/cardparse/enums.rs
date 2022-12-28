use std::{panic, collections::HashSet, fmt::Display};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
    Cleanse,
    CloseRange,
    Trample,
    Thorns,
    Slow,
    Scattershot,
    DoubleStrike,
    Scavenger,
    LifeLeech,
    TrueStrike,
    Repair,
    Demoralize,
    Dispel,
}

impl From<&str> for Ability {
    fn from(string: &str) -> Self {
        match string {
            "melee" => Self::Melee(0),
            "melee+" => Self::Melee(1),
            "melee-" => Self::Melee(-1),
            "ranged" => Self::Ranged(0),
            "ranged+" => Self::Ranged(1),
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
            "health-" => Self::Health(-1),
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
            "cleanse" => Self::Cleanse,
            "close range" => Self::CloseRange,
            "trample" => Self::Trample,
            "thorns" => Self::Thorns,
            "slow" => Self::Slow,
            "scattershot" => Self::Scattershot,
            "double strike" => Self::DoubleStrike,
            "scavenger" => Self::Scavenger,
            "life leech" => Self::LifeLeech,
            "true strike" => Self::TrueStrike,
            "repair" => Self::Repair,
            "demoralize" => Self::Demoralize,
            "dispel" => Self::Dispel,
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

    // pub fn extent_of(i: i8) -> i32 {
    //     if i < 0 {
    //         i as i32
    //     } else {
    //         i as i32
    //     }
    // }

    pub fn is_buff(&self) -> bool {
        match self {
            &Ability::Health(i) => i >= 0,
            &Ability::Armor(i) => i >= 0,
            &Ability::Speed(i) => i >= 0,
            &Ability::Melee(i) => i >= 0,
            &Ability::Ranged(i) => i >= 0,
            &Ability::Magic(i) => i >= 0,
            _ => false,
        }
    }

    pub fn is_debuff(&self) -> bool {
        match self {
            &Ability::Health(i) => i < 0,
            &Ability::Armor(i) => i < 0,
            &Ability::Speed(i) => i < 0,
            &Ability::Melee(i) => i < 0,
            &Ability::Ranged(i) => i < 0,
            &Ability::Magic(i) => i < 0,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    pub fn as_f32(self) -> f32 {
        match self {
            Self::Win => 1.0,
            Self::Draw => 0.5,
            Self::Lose => 0.0,
        }
    }
}
