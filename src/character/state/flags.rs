use crate::game_match::FlashType;
use crate::typedefs::collision::{Int, Vec2};
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

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize, Inspect, Default)]
pub struct Flags {
    pub melee: Hittable,
    pub bullet: Hittable,
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
    pub jump_start: bool,
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
}

fn default_friction() -> Int {
    0_50
}

#[derive(Debug, Clone, Copy)]
pub struct MovementData {
    pub accel: Vec2,
    pub vel: Vec2,
    pub pos: Vec2,
}
impl MovementData {
    pub fn new() -> Self {
        Self {
            accel: Vec2::zeros(),
            vel: Vec2::zeros(),
            pos: Vec2::zeros(),
        }
    }
}

impl Flags {
    pub fn new() -> Self {
        Self {
            melee: Hittable::Hit,
            bullet: Hittable::Hit,
            spirit_cost: 0,
            meter_cost: 0,
            spirit_delay: 0,
            reset_spirit_delay: false,
            can_block: false,
            grazing: false,
            airborne: false,
            jump_start: false,
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
        }
    }

    pub fn apply_movement(&self, mut value: MovementData) -> MovementData {
        if self.reset_velocity {
            value.vel = Vec2::zeros();
        }
        value.vel += self.accel;
        value.pos += value.vel;
        value
    }
}
