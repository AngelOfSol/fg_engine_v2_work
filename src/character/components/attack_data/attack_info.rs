use super::AttackLevel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttackInfo {
    pub level: AttackLevel,
    #[serde(default = "default_hitstop")]
    pub hitstop: i32,
}

fn default_hitstop() -> i32 {
    10
}

impl AttackInfo {
    pub fn new() -> Self {
        Self {
            level: AttackLevel::A,
            hitstop: default_hitstop(),
        }
    }
}

impl Default for AttackInfo {
    fn default() -> Self {
        Self::new()
    }
}
