mod attack;
mod attack_level;

pub use attack::AttackInfo;
pub use attack_level::AttackLevel;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Attacks {
    #[serde(flatten)]
    pub attacks: HashMap<String, AttackInfo>,
}

impl Attacks {
    pub fn new() -> Self {
        Self {
            attacks: HashMap::new(),
        }
    }
}
