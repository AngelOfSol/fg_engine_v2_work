use crate::game_match::FlashType;
use fg_datastructures::math::collision::{Int, Vec2};
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Inspect)]
pub enum Hittable {
    Invuln,
    Hit,
}

impl Default for Hittable {
    fn default() -> Self {
        Self::Hit
    }
}

impl Hittable {
    pub fn is_invuln(self) -> bool {
        match self {
            Hittable::Invuln => true,
            Hittable::Hit => false,
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize, Inspect)]
pub struct Flags {
    pub melee: Hittable,
    pub bullet: Hittable,
    #[serde(default)]
    pub air: Hittable,
    #[serde(default)]
    pub foot: Hittable,
    pub can_block: bool,
    #[serde(default)]
    pub grazing: bool,
    pub airborne: bool,
    pub reset_velocity: bool,

    #[serde(default)]
    pub crouching: bool,
    #[serde(default)]
    pub can_be_counter_hit: bool,
    #[serde(default)]
    pub spirit_cost: i32,
    #[serde(default)]
    pub spirit_delay: i32,
    #[serde(default)]
    pub reset_spirit_delay: bool,

    #[serde(default)]
    pub meter_cost: i32,

    #[serde(default)]
    pub allow_reface: bool,
    pub accel: Vec2,
    #[serde(default = "default_friction")]
    pub friction: Int,
    #[serde(default)]
    pub cutscene: bool,
    #[serde(default)]
    pub flash: Option<FlashType>,

    #[serde(default)]
    pub lockout_timer: i32,
    #[serde(default)]
    pub reset_lockout_timer: bool,
    #[serde(default = "default_gravity")]
    pub gravity: bool,
}

fn default_gravity() -> bool {
    true
}
fn default_friction() -> Int {
    0_50
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MovementData {
    pub accel: Vec2,
    pub vel: Vec2,
    pub pos: Vec2,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            melee: Hittable::Hit,
            bullet: Hittable::Hit,
            air: Hittable::Hit,
            foot: Hittable::Hit,
            spirit_cost: 0,
            meter_cost: 0,
            spirit_delay: 0,
            reset_spirit_delay: false,
            can_block: false,
            grazing: false,
            airborne: false,
            reset_velocity: false,
            allow_reface: false,
            crouching: false,
            can_be_counter_hit: false,
            accel: Vec2::zeros(),
            friction: default_friction(),
            cutscene: false,
            flash: None,
            lockout_timer: 0,
            reset_lockout_timer: false,
            gravity: true,
        }
    }
}
