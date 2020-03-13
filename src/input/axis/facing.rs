use crate::typedefs::{collision, graphics};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Facing {
    Left,
    Right,
}

impl Facing {
    pub fn fix_rotation(self, value: f32) -> f32 {
        match self {
            Facing::Left => {
                std::f32::consts::PI / 2.0
                    - value.signum() * (std::f32::consts::PI / 2.0 - value).abs()
            }
            Facing::Right => value,
        }
    }
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
