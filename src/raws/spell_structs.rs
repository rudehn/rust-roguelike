use serde::{Deserialize};
use std::collections::HashMap;
use super::EffectValues;

#[derive(Deserialize, Debug)]
pub struct Spell {
    pub name : String,
    pub mana_cost : i32,
    pub effects : HashMap<String, EffectValues>
}
