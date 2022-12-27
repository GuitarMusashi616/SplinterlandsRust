use crate::cardparse::enums::AttackType;

use super::{battledata::BattleData, monsterkey::MonsterKey};

pub fn print_attack(bd: &BattleData, mk: &MonsterKey, tk: &MonsterKey) {
    let monster = bd.get(mk).expect("mk not in bd");
    let target = bd.get(tk).expect("mk not in bd");
    println!("{} => -{} => {}", monster, monster.get_damage(), target);
}

pub fn attack(bd: &mut BattleData, mk: &MonsterKey, tk: &MonsterKey) {
    let monster = bd.get(mk).expect("mk not in bd");
    match monster.get_attack_type() {
        AttackType::Melee => attack_melee_or_ranged(bd, mk, tk),
        AttackType::Ranged => attack_melee_or_ranged(bd, mk, tk),
        AttackType::Magic => attack_magic(bd, mk, tk),
        AttackType::None => (),
    }
    print_attack(bd, mk, tk);
}

pub fn attack_melee_or_ranged(bd: &mut BattleData, mk: &MonsterKey, tk: &MonsterKey) {
    let monster = bd.get(mk).expect("mk not in bd");
    let damage = monster.get_damage();
    bd.deal_damage(tk, damage);
}

pub fn attack_magic(bd: &mut BattleData, mk: &MonsterKey, tk: &MonsterKey) {
    let monster = bd.get(mk).expect("mk not in bd");
    let damage = monster.get_damage();
    bd.deal_true_damage(tk, damage);
}