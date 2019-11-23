mod attack_info;
mod attack_level;
mod ground_action;
mod guard;

pub use attack_info::AttackInfo;
pub use attack_level::AttackLevel;
pub use ground_action::GroundAction;
pub use guard::Guard;
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
    pub fn guarentee_unique_key<S: Into<String>>(&self, key: S) -> String {
        let base = key.into();
        let mut new_key = base.clone();
        let mut counter = 1;
        loop {
            if self.attacks.contains_key(&new_key) {
                new_key = format!("{} ({})", base, counter);
                counter += 1;
            } else {
                break;
            }
        }
        new_key
    }
}
