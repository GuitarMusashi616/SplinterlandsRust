use rand::{thread_rng, Rng};

use crate::{cardparse::enums::{AttackType, Ability}, gamedata::monster::Monster};

use super::{battledata::BattleData, monsterkey::MonsterKey};

pub fn print_attack(bd: &BattleData, mk: &MonsterKey, tk: &MonsterKey) {
    let monster = bd.get(mk).expect("mk not in bd");
    let target = bd.get(tk).expect("mk not in bd");
    println!("{} => -{} => {}", monster, monster.get_damage(), target);
}

pub fn attack(bd: &mut BattleData, mk: &MonsterKey, tk: &MonsterKey) -> bool {
    let monster = bd.get(mk).expect("mk not in bd");
    match monster.get_attack_type() {
        AttackType::Melee => attack_melee_or_ranged(bd, mk, tk),
        AttackType::Ranged => attack_melee_or_ranged(bd, mk, tk),
        AttackType::Magic => attack_magic(bd, mk, tk),
        AttackType::None => false,
    }
    // print_attack(bd, mk, tk);
}

pub fn evade_check(monster: &Monster, target: &Monster) -> bool {
    let mut evade_chance: f32 = 0.0;
    let speed_diff = target.get_speed() - monster.get_speed();
    if speed_diff > 0 {
        evade_chance += 0.1 * speed_diff as f32;
    }

    if target.has_ability(Ability::Dodge) {
        evade_chance += 0.25;
    }

    if target.has_ability(Ability::Flying) && !monster.has_ability(Ability::Flying) {
        evade_chance += 0.25;
    }

    if monster.has_ability(Ability::TrueStrike) {
        evade_chance = 0.0;
    }

    let random: f32 = thread_rng().gen();
    if random < evade_chance {
        return true
    }
    false
}

pub fn attack_melee_or_ranged(bd: &mut BattleData, mk: &MonsterKey, tk: &MonsterKey) -> bool {
    let monster = bd.get(mk).expect("mk not in bd");
    let target = bd.get(tk).expect("tk not in bd");
    let mut damage = monster.get_damage();

    if target.has_ability(Ability::Shield) {
        if damage <= 1 {
            damage = 0
        } else {
            let step1 = damage as f32 / 2.0;
            damage = step1.ceil() as i32;
        }
    }

    if evade_check(monster, target) || damage <= 0 {
        return false;
    }
    bd.deal_damage(tk, damage);
    true
}

pub fn attack_magic(bd: &mut BattleData, mk: &MonsterKey, tk: &MonsterKey) -> bool {
    let monster = bd.get(mk).expect("mk not in bd");
    let target = bd.get(tk).expect("tk not in bd");
    let mut damage = monster.get_damage();

    if target.has_ability(Ability::Void) {
        if damage <= 1 {
            damage = 0
        } else {
            let step1 = damage as f32 / 2.0;
            damage = step1.ceil() as i32;
        }
    }

    if evade_check(monster, target) || damage <= 0 {
        return false;
    }
    bd.deal_true_damage(tk, damage);
    true
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_f32_ceil() {
        let step = (4.0 as f32 / 2.0 as f32).ceil();
        assert_eq!(step as i32, 2);
    }
}
