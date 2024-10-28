use serde::{Deserialize};
use std::collections::HashMap;
use super::EffectValues;

#[derive(Deserialize, Debug)]
pub struct WeaponTrait {
    pub name : String,
    pub effects : HashMap<String, EffectValues>
}
