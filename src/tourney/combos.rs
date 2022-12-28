use crate::gamedata::registry::Registry;
use crate::cardparse::enums::{Element, Role};
use itertools::Itertools;

pub fn deck_combos(reg: &Registry, elem: Element, mana_cost: i32) -> Vec<Vec<&String>> {
    let elem_mon = reg.filter(|card| (card.element == elem || card.element == Element::Neutral) && card.role == Role::Monster);
    let elem_summ = reg.filter(|card| (card.element == elem || card.element == Element::Neutral) && card.role == Role::Summoner);

    let mut valid = Vec::new();
    for summ in elem_summ {
        for combo in elem_mon.iter().combinations(5) {
            let val = combo.iter().fold(summ.1.mana_cost, |acc, (_, card)| acc + card.mana_cost);
            if val == mana_cost {
                let mut res = vec![summ.0];
                res.extend(combo.iter().map(|(name, _)| name));
                valid.push(res);
            }
        }
    }
    valid
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_combos() {
        let reg = Registry::from("assets/new_cards.csv");
        let valid = deck_combos(&reg, Element::Water, 16);

        for item in valid {
            println!("{:?}", item);
        }

    }
}

