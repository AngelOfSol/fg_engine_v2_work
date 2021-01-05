use crate::typedefs::{collision, graphics};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Facing {
    Left,
    Right,
}
impl Default for Facing {
    fn default() -> Self {
        Self::Right
    }
}

impl Facing {
    pub fn invert(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub fn fix_graphics(self, data: graphics::Vec2) -> graphics::Vec2 {
        data.component_mul(&self.graphics_multiplier())
    }
    pub fn graphics_multiplier(self) -> graphics::Vec2 {
        graphics::Vec2::new(
            match self {
                Self::Left => -1.0,
                Self::Right => 1.0,
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
                Self::Left => -1,
                Self::Right => 1,
            },
            1,
        )
    }
}
