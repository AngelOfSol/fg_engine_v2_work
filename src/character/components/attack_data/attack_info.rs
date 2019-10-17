use super::AttackLevel;
use crate::typedefs::collision::{Int, Vec2};
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

    #[serde(default = "default_air_force")]
    pub air_force: Vec2,
    #[serde(default = "default_ground_pushback")]
    pub ground_pushback: Int,
}

fn default_air_force() -> Vec2 {
    Vec2::new(4_00, 2_50)
}

fn default_ground_pushback() -> Int {
    10_00
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
            air_force: default_air_force(),
            ground_pushback: default_ground_pushback(),
        }
    }
}

impl Default for AttackInfo {
    fn default() -> Self {
        Self::new()
    }
}
