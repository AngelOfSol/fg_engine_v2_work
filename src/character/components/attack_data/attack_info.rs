use super::GroundAction;
use fg_datastructures::math::collision::{Int, Vec2};
use fg_input::guard::Guard;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Inspect)]
pub struct AttackInfo {
    pub melee: bool,
    pub magic: bool,
    #[serde(default)]
    pub air: bool,
    #[serde(default)]
    pub foot: bool,
    pub guard: Guard,
    pub air_unblockable: bool,
    pub can_counter_hit: bool,
    pub grazeable: bool,

    #[tab = "On Graze"]
    pub on_graze: GrazeInfo,
    #[tab = "On CH"]
    pub on_counter_hit: CounterHitInfo,
    #[tab = "On Crush"]
    pub on_guard_crush: GuardCrushInfo,
    #[tab = "On Hit"]
    pub on_hit: HitInfo,
    #[tab = "On Block"]
    pub on_block: BlockInfo,
    #[tab = "On Wrongblock"]
    pub on_wrongblock: WrongBlockInfo,
}

impl Default for AttackInfo {
    fn default() -> Self {
        Self {
            melee: true,
            magic: false,
            air: false,
            foot: false,
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Inspect)]
pub struct GrazeInfo {
    pub defender_stop: i32,
    pub damage: i32,
    pub spirit_cost: i32,
    pub spirit_delay: i32,
    pub reset_spirit_delay: bool,
    #[serde(default = "default_graze_attacker_meter")]
    pub attacker_meter: i32,
    #[serde(default = "default_graze_defender_meter")]
    pub defender_meter: i32,
}

fn default_graze_attacker_meter() -> i32 {
    -1_00
}

fn default_graze_defender_meter() -> i32 {
    1_00
}

impl Default for GrazeInfo {
    fn default() -> Self {
        Self {
            defender_stop: 0,
            damage: 0,
            spirit_cost: 0,
            spirit_delay: 0,
            reset_spirit_delay: false,
            attacker_meter: default_graze_attacker_meter(),
            defender_meter: default_graze_defender_meter(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Inspect)]
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
    #[serde(default = "default_hit_attacker_meter")]
    pub attacker_meter: i32,
    #[serde(default = "default_hit_defender_meter")]
    pub defender_meter: i32,
}

fn default_hit_attacker_meter() -> i32 {
    3_00
}

fn default_hit_defender_meter() -> i32 {
    1_00
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
            attacker_meter: default_hit_attacker_meter(),
            defender_meter: default_hit_defender_meter(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Inspect)]
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
    #[serde(default = "default_counter_hit_attacker_meter")]
    pub attacker_meter: i32,
    #[serde(default = "default_counter_hit_defender_meter")]
    pub defender_meter: i32,
}
fn default_counter_hit_attacker_meter() -> i32 {
    5_00
}

fn default_counter_hit_defender_meter() -> i32 {
    0_00
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
            attacker_meter: default_counter_hit_attacker_meter(),
            defender_meter: default_counter_hit_defender_meter(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Inspect)]
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

    #[serde(default = "default_guard_crush_attacker_meter")]
    pub attacker_meter: i32,
    #[serde(default = "default_guard_crush_defender_meter")]
    pub defender_meter: i32,
}
fn default_guard_crush_attacker_meter() -> i32 {
    8_00
}

fn default_guard_crush_defender_meter() -> i32 {
    0_00
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
            attacker_meter: default_guard_crush_attacker_meter(),
            defender_meter: default_guard_crush_defender_meter(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Inspect)]
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

    #[serde(default = "default_block_attacker_meter")]
    pub attacker_meter: i32,
    #[serde(default = "default_block_defender_meter")]
    pub defender_meter: i32,
}
fn default_block_attacker_meter() -> i32 {
    0_00
}

fn default_block_defender_meter() -> i32 {
    3_00
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
            attacker_meter: default_block_attacker_meter(),
            defender_meter: default_block_defender_meter(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Inspect)]
pub struct WrongBlockInfo {
    pub attacker_stop: i32,
    pub defender_stop: i32,
    pub stun: i32,
    pub damage: i32,
    pub spirit_cost: i32,
    pub spirit_delay: i32,
    pub reset_spirit_delay: bool,
    pub ground_pushback: Int,

    #[serde(default = "default_wrong_block_attacker_meter")]
    pub attacker_meter: i32,
    #[serde(default = "default_wrong_block_defender_meter")]
    pub defender_meter: i32,
}
fn default_wrong_block_attacker_meter() -> i32 {
    5_00
}

fn default_wrong_block_defender_meter() -> i32 {
    -3_00
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
            attacker_meter: default_wrong_block_attacker_meter(),
            defender_meter: default_wrong_block_defender_meter(),
        }
    }
}

fn default_air_force() -> Vec2 {
    Vec2::new(4_00, 2_50)
}

fn default_ground_pushback() -> Int {
    10_00
}
