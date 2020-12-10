use crate::typedefs::{collision, graphics};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Facing {
    Left,
    Right,
}

impl Facing {
    pub fn invert(self) -> Self {
        match self {
            Facing::Left => Facing::Right,
            Facing::Right => Facing::Left,
        }
    }

    pub fn fix_graphics(self, data: graphics::Vec2) -> graphics::Vec2 {
        data.component_mul(&self.graphics_multiplier())
    }
    pub fn graphics_multiplier(self) -> graphics::Vec2 {
        graphics::Vec2::new(
            match self {
                Facing::Left => -1.0,
                Facing::Right => 1.0,
            },
            1.0,
        )
    }
    pub fn fix_collision(self, data: collision::Vec2) -> collision::Vec2 {
        data.component_mul(&self.collision_multiplier())
    }
    pub fn collision_multiplier(self) -> collision::Vec2 {
        collision::Vec2::new(
            match self {
                Facing::Left => -1,
                Facing::Right => 1,
            },
            1,
        )
    }
}
