use super::AttackLevel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttackInfo {
    pub level: AttackLevel,
    #[serde(default = "default_hitstop")]
    pub defender_hitstop: i32,
    #[serde(default = "default_hitstop")]
    pub defender_blockstop: i32,
    #[serde(default = "default_hitstop")]
    pub attacker_hitstop: i32,
    #[serde(default = "default_hitstop")]
    pub attacker_blockstop: i32,
}

fn default_hitstop() -> i32 {
    10
}

impl AttackInfo {
    pub fn new() -> Self {
        Self {
            level: AttackLevel::A,
            defender_hitstop: default_hitstop(),
            defender_blockstop: default_hitstop(),
            attacker_hitstop: default_hitstop(),
            attacker_blockstop: default_hitstop(),
        }
    }
}

impl Default for AttackInfo {
    fn default() -> Self {
        Self::new()
    }
}
