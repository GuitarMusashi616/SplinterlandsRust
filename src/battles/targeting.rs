use rand::{thread_rng, seq::SliceRandom};

use crate::cardparse::enums::{AttackType, Ability};

use super::{battledata::BattleData, monsterkey::MonsterKey};

pub fn target_for(bd: &BattleData, mk: &MonsterKey) -> Option<MonsterKey> {
    // initial checks
    if bd.oppo_alive.is_empty() || bd.home_alive.is_empty() {
        return None;
    }

    let monster = bd.get(mk).expect(&format!("{:?} not in battledata", mk));
    if !monster.is_alive() {
        return None;
    }

    // match the type of attack
    match monster.get_attack_type() {
        AttackType::Melee => target_for_melee(bd, mk),
        AttackType::Ranged => target_for_ranged(bd, mk),
        AttackType::Magic => target_for_magic(bd, mk),
        AttackType::None => None,
    }
}

pub fn check_taunt(bd: &BattleData, mk: &MonsterKey) -> Option<MonsterKey> {
    match mk {
        MonsterKey::Home(_) => bd.oppo_alive.get_taunt(bd),
        MonsterKey::Oppo(_) => bd.home_alive.get_taunt(bd)
    }
}

pub fn target_for_sneak(bd: &BattleData, mk: &MonsterKey) -> Option<MonsterKey> {
    let oppo_last = bd.oppo_alive.len() - 1;
    let home_last = bd.home_alive.len() - 1;

    let taunt = check_taunt(bd, mk);
    if taunt.is_some() {
        return taunt;
    }

    match mk {
        MonsterKey::Home(_) => Some(bd.oppo_alive.index(oppo_last)),
        MonsterKey::Oppo(_) => Some(bd.home_alive.index(home_last))
    }
}

pub fn target_for_opportunity(bd: &BattleData, mk: &MonsterKey) -> Option<MonsterKey> {
    if let Some(taunt) = check_taunt(bd, mk) {
        return Some(taunt);
    }
    
    let taunt = check_taunt(bd, mk);
    if taunt.is_some() {
        return taunt;
    }

    match mk {
        MonsterKey::Home(_) => bd.oppo_alive.least_health(bd),
        MonsterKey::Oppo(_) => bd.home_alive.least_health(bd)
    }
}

pub fn target_for_melee(bd: &BattleData, mk: &MonsterKey) -> Option<MonsterKey> {
    let monster = bd.get(mk).expect("mk is not in battle");

    let mk_pos =  bd.get_pos(mk).expect("mk is not alive");
    let in_1st_pos = mk_pos == 0;

    if !in_1st_pos && monster.has_ability(Ability::Sneak) {
        return target_for_sneak(bd, mk);
    }

    if !in_1st_pos && monster.has_ability(Ability::Opportunity) {
        return target_for_opportunity(bd, mk);
    }

    let in_2nd_pos_with_reach = mk_pos == 1 && monster.has_ability(Ability::Reach);
    if !in_1st_pos && !in_2nd_pos_with_reach {
        return None;
    }
    target_first_pos(bd, mk)
}

/// Get non melee non 1st pos enemy
pub fn target_for_snipe(bd: &BattleData, mk: &MonsterKey) -> Option<MonsterKey> {
    match mk {
        MonsterKey::Home(_) => bd.oppo_alive.first_from_filter(|tk| {
            let target = bd.get(tk).unwrap();
            target.get_attack_type() != AttackType::Melee &&
            bd.get_pos(tk).unwrap_or(0) != 0
        }),
        MonsterKey::Oppo(_) => bd.home_alive.first_from_filter(|tk| {
            let target = bd.get(tk).unwrap();
            target.get_attack_type() != AttackType::Melee &&
            bd.get_pos(tk).unwrap_or(0) != 0
        }),
    }
}

pub fn target_for_ranged(bd: &BattleData, mk: &MonsterKey) -> Option<MonsterKey> {
    let monster = bd.get(mk).expect("mk is not in battle");

    let in_1st_pos = bd.get_pos(mk).expect("mk is not alive") == 0;

    if in_1st_pos && monster.has_ability(Ability::CloseRange) {
        return target_first_pos(bd, mk)
    }

    if in_1st_pos {
        return None;
    }

    let taunt = check_taunt(bd, mk);
    if taunt.is_some() {
        return taunt;
    }

    if monster.has_ability(Ability::Snipe) {
        let res = target_for_snipe(bd, mk);
        if res.is_some() {
            return res;
        }
    }

    target_first_pos(bd, mk)
}

pub fn target_for_magic(bd: &BattleData, mk: &MonsterKey) -> Option<MonsterKey> {
    let in_1st_pos = bd.get_pos(mk).expect("mk is not alive") == 0;
    if !in_1st_pos {
        let taunt = check_taunt(bd, mk);
        if taunt.is_some() {
            return taunt;
        }
    }
    target_first_pos(bd, mk)
}

pub fn target_random(bd: &BattleData, mk: &MonsterKey) -> Option<MonsterKey> {
    let mut rng = thread_rng();
    match mk {
        MonsterKey::Home(_) => Some(*bd.oppo_alive.choose(&mut rng).unwrap()),
        MonsterKey::Oppo(_) => Some(*bd.home_alive.choose(&mut rng).unwrap())
    }
}

pub fn target_first_pos(bd: &BattleData, mk: &MonsterKey) -> Option<MonsterKey> {
    match mk {
        MonsterKey::Home(_) => Some(bd.oppo_alive.index(0)),
        MonsterKey::Oppo(_) => Some(bd.home_alive.index(0))
    }
}
