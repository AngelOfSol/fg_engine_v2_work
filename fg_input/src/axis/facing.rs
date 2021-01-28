use std::ops::Neg;

use nalgebra::{Scalar, Vector2};
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
    pub fn fix<T: Scalar + Neg<Output = T> + Copy>(self, mut data: Vector2<T>) -> Vector2<T> {
        match self {
            Facing::Left => {
                data.x = -data.x;
                data
            }
            Facing::Right => data,
        }
    }
}
