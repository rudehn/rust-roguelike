use specs::prelude::Entity;
use super::{Skill, Skills};

pub fn attr_bonus(value: i32) -> i32 {
    (value-10)/2 // See: https://roll20.net/compendium/dnd5e/Ability%20Scores#content
}

pub fn player_hp_per_level(constitution: i32) -> i32 {
    15 + attr_bonus(constitution)
}

pub fn player_hp_at_level(constitution:i32, level:i32) -> i32 {
    15 + (player_hp_per_level(constitution) * level)
}

pub fn mana_per_level(intelligence: i32) -> i32 {
    i32::max(1, 4 + attr_bonus(intelligence))
}

pub fn mana_at_level(intelligence: i32, level: i32) -> i32 {
    mana_per_level(intelligence) * i32::max(1, level)
}

pub fn skill_bonus(skill : Skill, skills: &Skills) -> i32 {
    if skills.skills.contains_key(&skill) {
        skills.skills[&skill]
    } else {
        -4
    }
}

pub fn saving_throw(bonus: i32) -> i32 {
    let nat_roll = crate::rng::roll_dice(1, 20);
    return nat_roll + bonus;
}

pub fn xp_to_next_level(level: i32) -> i32 {
    // Return the amount of exp needed to get to the next level
    let base_exp = 100;
    let exponent = 1.5;
    return base_exp * (((level - 1) as f32).powf(exponent) as i32);
}
