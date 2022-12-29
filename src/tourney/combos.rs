use std::ops::Deref;

use crate::battles::battle::Battle;
use crate::gamedata::registry::Registry;
use crate::cardparse::enums::{Element, Role, Ability};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;

use super::elo::Elo;

// pub fn deck_combos(reg: &Registry, elem: Element, mana_cost: i32) -> Vec<Vec<&str>> {
//     let elem_mon = reg.filter(|card| (card.element == elem || card.element == Element::Neutral) && card.role == Role::Monster);
//     let elem_summ = reg.filter(|card| (card.element == elem || card.element == Element::Neutral) && card.role == Role::Summoner);

//     let mut valid = Vec::new();
//     for summ in elem_summ {
//         for combo in elem_mon.iter().combinations(5) {
//             let val = combo.iter().fold(summ.1.mana_cost, |acc, (_, card)| acc + card.mana_cost);
//             if val == mana_cost {
//                 let mut res = vec![summ.0];
//                 res.extend(combo.iter().map(|(name, _)| name));
//                 valid.push(res);
//             }
//         }
//     }
//     valid
// }

pub fn elo_combos(reg: &Registry, elem: Element, mana_cost: i32) -> Vec<Elo> {
    let elem_mon = reg.filter(|card| (card.element == elem || card.element == Element::Neutral) && card.role == Role::Monster);
    let elem_summ = reg.filter(|card| (card.element == elem || card.element == Element::Neutral) && card.role == Role::Summoner);

    let mut valid = Vec::new();
    for summ in elem_summ {
        for combo in elem_mon.iter().combinations(5) {
            let val = combo.iter().fold(summ.1.mana_cost, |acc, (_, card)| acc + card.mana_cost);
            if val == mana_cost {
                let mut res = vec![summ.0];
                res.extend(combo.iter().map(|(name, _)| name));
                valid.push(Elo::new(res));
            }
        }
    }
    valid
}

// pub fn deck_combos(reg: &Registry, elem: Element, mana_cost: i32, mut f: impl FnMut(Elo)) {
//     let elem_mon = reg.filter(|card| (card.element == elem || card.element == Element::Neutral) && card.role == Role::Monster);
//     let elem_summ = reg.filter(|card| (card.element == elem || card.element == Element::Neutral) && card.role == Role::Summoner);

//     for summ in elem_summ {
//         for combo in elem_mon.iter().combinations(5) {
//             let val = combo.iter().fold(summ.1.mana_cost, |acc, (_, card)| acc + card.mana_cost);
//             if val == mana_cost {
//                 let mut res = vec![summ.0];
//                 res.extend(combo.iter().map(|(name, _)| name));
//                 f(Elo::new(res))
//             }
//         }
//     }
// }

// pub fn deck_combos<'a>(reg: &'a Registry, elem: Element, mana_cost: i32, collector: &'a mut Vec<Elo<'a>>) {
//     let elem_mon = reg.filter(|card| (card.element == elem || card.element == Element::Neutral) && card.role == Role::Monster);
//     let elem_summ = reg.filter(|card| (card.element == elem || card.element == Element::Neutral) && card.role == Role::Summoner);

//     for summ in elem_summ {
//         for combo in elem_mon.iter().combinations(5) {
//             let val = combo.iter().fold(summ.1.mana_cost, |acc, (_, card)| acc + card.mana_cost);
//             if val == mana_cost {
//                 let mut res = vec![summ.0];
//                 res.extend(combo.iter().map(|(name, _)| name));
//                 collector.push(Elo::new(res));
//             }
//         }
//     }
// }

// pub fn get_pairs<'a>(combos: &'a Vec<Vec<&'a str>>) -> impl Iterator<Item = (&'a Vec<&'a str>, &'a Vec<&'a str>)> {
//     let half = combos.len() / 2;

//     combos.iter().take(half).zip(combos.iter().skip(half))
// }

// pub fn into_pairs(combos: Vec<Vec<&String>>) -> impl Iterator<Item = (Vec<&String>, Vec<&STring>)

pub fn battle_in_pairs(elos: &mut [Elo], reg: &Registry) {
    elos.iter_mut().fold(None, |acc, elem| {
        match acc {
            None => Some(elem),
            Some(prev) => {
                prev.battle(elem, reg);
                None
            }
        }
    });
}

pub fn tournament(reg: &Registry, elem: Element, mana_cost: i32, train: i32, lines: usize) {
    let mut elos = elo_combos(reg, elem, mana_cost);

    training(reg, &mut elos, train, lines);
}

pub fn super_tournament(reg: &Registry, mana_cost: i32, train: i32, lines: usize) {
    let mut elos = Vec::new();

    for elem in [Element::Fire, Element::Water, Element::Earth, Element::Life, Element::Death] {
        elos.push(elo_combos(reg, elem, mana_cost));
    }
    let mut elos = elos.into_iter().flatten().collect();

    training(reg, &mut elos, train, lines);
}

pub fn cut_lt(elos: &mut Vec<Elo>, cutoff: f32) {
    while elos.last().map(|x|x.elo).unwrap_or(cutoff) < cutoff {
        elos.pop();
    }
}

pub fn training(reg: &Registry, elos: &mut Vec<Elo>, train: i32, lines: usize) {
    for _ in 0..train {
        battle_in_pairs(elos, reg);
        elos.sort();
        cut_lt(elos, 1000.0);
    }

    elos.iter_mut().take(lines).for_each(|elo| {
        println!("{}", elo);
    });
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_combos() {
        let reg = Registry::from("assets/new_cards.csv");
        
        let valid = elo_combos(&reg, Element::Water, 20);

        for item in valid {
            println!("{:?}", item);
        }
    }

    fn test_tourney() {
        let reg = Registry::from("assets/new_cards.csv");
        tournament(&reg, Element::Death, 16, 10, usize::MAX);
    }
}

