pub mod directed_axis;
pub mod direction;
pub mod facing;

pub use directed_axis::DirectedAxis;
pub use direction::Direction;
pub use facing::Facing;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, Display)]
pub enum Axis {
    Up,
    Down,
    Right,
    Left,
    Neutral,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Axis {
    pub fn shift_down(self) -> Self {
        match self {
            Self::Up => Self::Neutral,
            Self::Right => Self::DownRight,
            Self::Left => Self::DownLeft,
            Self::Neutral => Self::Down,
            Self::UpRight => Self::Right,
            Self::UpLeft => Self::Left,
            _ => self,
        }
    }
    pub fn shift_up(self) -> Self {
        match self {
            Self::Down => Self::Neutral,
            Self::Right => Self::UpRight,
            Self::Left => Self::UpLeft,
            Self::Neutral => Self::Up,
            Self::DownRight => Self::Right,
            Self::DownLeft => Self::Left,
            _ => self,
        }
    }
}

impl From<[i32; 2]> for Axis {
    fn from([x, y]: [i32; 2]) -> Self {
        match x.cmp(&0) {
            std::cmp::Ordering::Greater => match y.cmp(&0) {
                std::cmp::Ordering::Greater => Axis::UpRight,
                std::cmp::Ordering::Less => Axis::DownRight,
                std::cmp::Ordering::Equal => Axis::Right,
            },
            std::cmp::Ordering::Less => match y.cmp(&0) {
                std::cmp::Ordering::Greater => Axis::UpLeft,
                std::cmp::Ordering::Less => Axis::DownLeft,
                std::cmp::Ordering::Equal => Axis::Left,
            },
            std::cmp::Ordering::Equal => match y.cmp(&0) {
                std::cmp::Ordering::Greater => Axis::Up,
                std::cmp::Ordering::Less => Axis::Down,
                std::cmp::Ordering::Equal => Axis::Neutral,
            },
        }
    }
}
