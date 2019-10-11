use crate::input::Facing;
use crate::typedefs::collision::{Int, Vec2};
use crate::typedefs::graphics::Float;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BulletId {
    Butterfly,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "bullet_id")]
pub enum BulletSpawn {
    Butterfly {
        offset: Vec2,
        angle: Int,
        x_vel: Int,
        y_vel: Int,
        frame: usize,
    },
}
#[derive(Clone, Copy, Debug)]
pub enum BulletState {
    Butterfly {
        position: Vec2,
        velocity: Vec2,
        rotation: Float,
    },
}

impl BulletSpawn {
    pub fn get_spawn_frame(&self) -> usize {
        match self {
            BulletSpawn::Butterfly { frame, .. } => *frame,
        }
    }
    pub fn instantiate(&self, current_position: Vec2, facing: Facing) -> BulletState {
        match self {
            BulletSpawn::Butterfly {
                offset,
                angle,
                x_vel,
                y_vel,
                ..
            } => BulletState::Butterfly {
                position: current_position + facing.fix_collision(*offset),
                velocity: facing.fix_collision(Vec2::new(*x_vel, *y_vel)),
                rotation: facing.fix_rotation(*angle as f32 * std::f32::consts::PI / -180.0),
            },
        }
    }
}

impl Default for BulletId {
    fn default() -> Self {
        BulletId::Butterfly
    }
}
impl Default for BulletSpawn {
    fn default() -> Self {
        panic!("called unnecessary default for BulletSpawn");
    }
}
