use super::{AttackLevel, GroundAction, Guard};
use crate::typedefs::collision::{Int, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct AttackInfo {
    pub level: AttackLevel,
    #[serde(default)]
    pub guard: Guard,
    #[serde(default)]
    pub air_unblockable: bool,
    #[serde(default)]
    pub grazeable: bool,
    #[serde(default)]
    pub melee: bool,

    #[serde(default)]
    pub on_hit: HitInfo,

    #[serde(default)]
    pub hit_damage: i32,
    #[serde(default)]
    pub proration: i32,
    #[serde(default)]
    pub launcher: bool,
    #[serde(default)]
    pub ground_action: GroundAction,

    #[serde(default)]
    pub starter_limit: i32,
    #[serde(default)]
    pub limit_cost: i32,

    #[serde(default)]
    pub counter_hit_limit: i32,
    #[serde(default)]
    pub can_counter_hit: bool,

    #[serde(default)]
    pub on_block: HitInfo,

    #[serde(default)]
    pub spirit_cost: i32,
    #[serde(default)]
    pub spirit_delay: i32,
    #[serde(default)]
    pub reset_spirit_delay: bool,
    #[serde(default)]
    pub chip_damage: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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
            air_unblockable: false,
            grazeable: false,
            melee: false,
            launcher: false,
            ground_action: GroundAction::default(),
            spirit_cost: 0,
            spirit_delay: 0,
            reset_spirit_delay: false,
            hit_damage: 200,
            proration: 100,
            chip_damage: 0,
            starter_limit: 100,
            limit_cost: 30,
            counter_hit_limit: 120,
            can_counter_hit: false,
        }
    }
}

impl Default for AttackInfo {
    fn default() -> Self {
        Self::new()
    }
}
