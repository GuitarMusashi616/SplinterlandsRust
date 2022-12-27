use rand::{thread_rng, seq::SliceRandom};

use crate::cardparse::enums::{AttackType, Ability};

use super::{battledata::BattleData, monsterkey::MonsterKey};

pub fn target_for(bd: &BattleData, mk: &MonsterKey) -> Option<MonsterKey> {
    // path of melee
    let monster = bd.get(mk).expect("mk not in battledata");
    match monster.get_attack_type() {
        &AttackType::Melee => target_for_melee(bd, mk),
        &AttackType::Ranged => target_for_ranged(bd, mk),
        &AttackType::Magic => target_for_magic(bd, mk),
        &AttackType::None => None,
    }
}

pub fn target_for_melee(bd: &BattleData, mk: &MonsterKey) -> Option<MonsterKey> {
    if bd.oppo_alive.is_empty() || bd.home_alive.is_empty() {
        return None;
    }
    let monster = bd.get(mk).expect("mk is not in battle");
    let mk_pos =  bd.get_pos(mk).expect("mk is not alive");
    let in_1st_pos = mk_pos == 0;
    let in_2nd_pos_with_reach = mk_pos == 1 && monster.has_ability(Ability::Reach);
    if !in_1st_pos && !in_2nd_pos_with_reach {
        return None;
    }
    match mk {
        MonsterKey::Home(_) => Some(bd.oppo_alive.index(0)),
        MonsterKey::Oppo(_) => Some(bd.home_alive.index(0))
    }
}

pub fn target_for_ranged(bd: &BattleData, mk: &MonsterKey) -> Option<MonsterKey> {
    if bd.oppo_alive.is_empty() || bd.home_alive.is_empty() {
        return None;
    }
    let monster = bd.get(mk).expect("mk is not in battle");
    let in_1st_pos = bd.get_pos(mk).expect("mk is not alive") == 0;
    if in_1st_pos {
        return None;
    }
    let mut rng = thread_rng();
    match mk {
        MonsterKey::Home(_) => Some(*bd.oppo_alive.choose(&mut rng).unwrap()),
        MonsterKey::Oppo(_) => Some(*bd.home_alive.choose(&mut rng).unwrap())
    }
}

pub fn target_for_magic(bd: &BattleData, mk: &MonsterKey) -> Option<MonsterKey> {
    if bd.oppo_alive.is_empty() || bd.home_alive.is_empty() {
        return None;
    }
    let monster = bd.get(mk).expect("mk is not in battle");
    let mut rng = thread_rng();
    match mk {
        MonsterKey::Home(_) => Some(*bd.oppo_alive.choose(&mut rng).unwrap()),
        MonsterKey::Oppo(_) => Some(*bd.home_alive.choose(&mut rng).unwrap())
    }
}