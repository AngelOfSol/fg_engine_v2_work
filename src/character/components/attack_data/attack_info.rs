use super::{AttackLevel, Guard};
use crate::typedefs::collision::{Int, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttackInfo {
    pub level: AttackLevel,
    #[serde(default)]
    pub guard: Guard,
    #[serde(default)]
    pub on_hit: HitInfo,
    #[serde(default)]
    pub on_block: HitInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HitInfo {
    #[serde(default = "default_hitstop")]
    pub attacker_stop: i32,
    #[serde(default = "default_hitstop")]
    pub defender_stop: i32,
    #[serde(default = "default_air_force")]
    pub air_force: Vec2,
    #[serde(default = "default_ground_pushback")]
    pub ground_pushback: Int,
}

impl Default for HitInfo {
    fn default() -> Self {
        Self {
            attacker_stop: default_hitstop(),
            defender_stop: default_hitstop(),
            air_force: default_air_force(),
            ground_pushback: default_ground_pushback(),
        }
    }
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
            guard: Default::default(),
            on_hit: Default::default(),
            on_block: Default::default(),
        }
    }
}

impl Default for AttackInfo {
    fn default() -> Self {
        Self::new()
    }
}
