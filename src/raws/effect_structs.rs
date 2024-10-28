use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct EffectValues {
    pub amount: Option<i32>,
    pub duration: Option<i32>,
    pub value: Option<String>
}