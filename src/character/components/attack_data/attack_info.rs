use super::{GroundAction, Guard};
use crate::typedefs::collision::{Int, Vec2};
use serde::{Deserialize, Serialize};

pub mod version;

pub type AttackInfo = AttackInfoV1;

impl AttackInfoV1 {
    pub fn to_modern(self) -> AttackInfoV1 {
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct AttackInfoV1 {
    pub melee: bool,
    pub magic: bool,
    pub guard: Guard,
    pub air_unblockable: bool,
    pub can_counter_hit: bool,
    pub grazeable: bool,

    pub on_graze: GrazeInfo,
    pub on_counter_hit: CounterHitInfo,
    pub on_guard_crush: GuardCrushInfo,
    pub on_hit: HitInfo,
    pub on_block: BlockInfo,
    pub on_wrongblock: WrongBlockInfo,
}


impl Default for AttackInfoV1 {
    fn default() -> Self {
        Self {
            melee: true,
            magic: false,
            guard: Guard::Mid,
            air_unblockable: false,
            can_counter_hit: false,
            grazeable: false,
            on_graze: GrazeInfo::default(),
            on_counter_hit: CounterHitInfo::default(),
            on_guard_crush: GuardCrushInfo::default(),
            on_hit: HitInfo::default(),
            on_block: BlockInfo::default(),
            on_wrongblock: WrongBlockInfo::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct GrazeInfo {
    pub defender_stop: i32,
    pub damage: i32,
    pub spirit_cost: i32,
    pub spirit_delay: i32,
    pub reset_spirit_delay: bool,
}

impl Default for GrazeInfo {
    fn default() -> Self {
        Self {
            defender_stop: 0,
            damage: 0,
            spirit_cost: 0,
            spirit_delay: 0,
            reset_spirit_delay: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct HitInfo {
    pub attacker_stop: i32,
    pub defender_stop: i32,
    pub stun: i32,
    pub air_stun: i32,
    pub damage: i32,
    pub lethal: bool,
    pub spirit_cost: i32,
    pub spirit_delay: i32,
    pub reset_spirit_delay: bool,
    pub air_force: Vec2,
    pub ground_pushback: Int,
    pub launcher: bool,
    pub ground_action: GroundAction,
    pub starter_limit: i32,
    pub limit_cost: i32,
    pub proration: i32,
}

impl Default for HitInfo {
    fn default() -> Self {
        Self {
            attacker_stop: 10,
            defender_stop: 10,
            stun: 13,
            air_stun: 26,
            damage: 100,
            lethal: true,
            spirit_cost: 0,
            spirit_delay: 0,
            reset_spirit_delay: false,
            air_force: default_air_force(),
            ground_pushback: default_ground_pushback(),
            launcher: false,
            ground_action: GroundAction::Knockdown,
            starter_limit: 50,
            limit_cost: 10,
            proration: 80,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct CounterHitInfo {
    pub attacker_stop: i32,
    pub defender_stop: i32,
    pub stun: i32,
    pub air_stun: i32,
    pub damage: i32,
    pub lethal: bool,
    pub spirit_cost: i32,
    pub spirit_delay: i32,
    pub reset_spirit_delay: bool,
    pub air_force: Vec2,
    pub ground_pushback: Int,
    pub launcher: bool,
    pub ground_action: GroundAction,
    pub starter_limit: i32,
    pub proration: i32,
}
impl Default for CounterHitInfo {
    fn default() -> Self {
        Self {
            attacker_stop: 10,
            defender_stop: 10,
            stun: 18,
            air_stun: 36,
            damage: 100,
            lethal: true,
            spirit_cost: 0,
            spirit_delay: 0,
            reset_spirit_delay: false,
            air_force: default_air_force(),
            ground_pushback: default_ground_pushback(),
            launcher: false,
            ground_action: GroundAction::Knockdown,
            starter_limit: 70,
            proration: 95,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct GuardCrushInfo {
    pub attacker_stop: i32,
    pub defender_stop: i32,
    pub stun: i32,
    pub air_stun: i32,
    pub damage: i32,
    pub lethal: bool,
    pub air_force: Vec2,
    pub ground_pushback: Int,
    pub launcher: bool,
    pub ground_action: GroundAction,
    pub starter_limit: i32,
    pub proration: i32,
}
impl Default for GuardCrushInfo {
    fn default() -> Self {
        Self {
            attacker_stop: 10,
            defender_stop: 10,
            stun: 55,
            air_stun: 55,
            damage: 0,
            lethal: true,
            air_force: default_air_force(),
            ground_pushback: default_ground_pushback(),
            launcher: false,
            ground_action: GroundAction::Knockdown,
            starter_limit: 70,
            proration: 80,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct BlockInfo {
    pub attacker_stop: i32,
    pub defender_stop: i32,
    pub stun: i32,
    pub air_stun: i32,
    pub damage: i32,
    pub spirit_cost: i32,
    pub spirit_delay: i32,
    pub reset_spirit_delay: bool,
    pub air_force: Vec2,
    pub ground_pushback: Int,
}
impl Default for BlockInfo {
    fn default() -> Self {
        Self {
            attacker_stop: 10,
            defender_stop: 10,
            stun: 11,
            air_stun: 22,
            damage: 0,
            spirit_cost: 0,
            spirit_delay: 0,
            reset_spirit_delay: false,
            air_force: default_air_force(),
            ground_pushback: default_ground_pushback(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct WrongBlockInfo {
    pub attacker_stop: i32,
    pub defender_stop: i32,
    pub stun: i32,
    pub damage: i32,
    pub spirit_cost: i32,
    pub spirit_delay: i32,
    pub reset_spirit_delay: bool,
    pub ground_pushback: Int,
}

impl Default for WrongBlockInfo {
    fn default() -> Self {
        Self {
            attacker_stop: 10,
            defender_stop: 10,
            stun: 15,
            damage: 0,
            spirit_cost: 100,
            spirit_delay: 60,
            reset_spirit_delay: true,
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
