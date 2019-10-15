use crate::typedefs::collision::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum MeleeHittable {
    Invuln,
    Hit,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum MagicHittable {
    Hit,
    Graze,
    Invuln,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct Flags {
    pub melee: MeleeHittable,
    pub bullet: MagicHittable,
    pub can_block: bool,
    pub airborne: bool,
    pub reset_velocity: bool,

    #[serde(default)]
    pub spirit_cost: i32,
    #[serde(default)]
    pub spirit_delay: i32,
    #[serde(default)]
    pub reset_spirit_delay: bool,

    #[serde(default)]
    pub jump_start: bool,
    #[serde(default)]
    pub allow_reface: bool,
    pub accel: Vec2,
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
            melee: MeleeHittable::Hit,
            bullet: MagicHittable::Hit,
            spirit_cost: 0,
            spirit_delay: 0,
            reset_spirit_delay: false,
            can_block: false,
            airborne: false,
            jump_start: false,
            reset_velocity: true,
            allow_reface: false,
            accel: Vec2::zeros(),
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
