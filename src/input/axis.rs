mod directed_axis;
mod direction;
mod facing;

pub use directed_axis::DirectedAxis;
pub use direction::Direction;
pub use facing::Facing;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

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

impl From<[i8; 2]> for Axis {
    fn from([x, y]: [i8; 2]) -> Self {
        if x > 0 {
            Axis::Right
        } else if x < 0 {
            Axis::Left
        } else {
            Axis::Neutral
        }
        .add(if y > 0 {
            Axis::Up
        } else if y < 0 {
            Axis::Down
        } else {
            Axis::Neutral
        })
    }
}

impl Axis {
    pub fn add(self, new: Axis) -> Self {
        match new {
            Axis::UpRight | Axis::UpLeft | Axis::DownRight | Axis::DownLeft => {
                panic!("Adding diagonal doesn't make sense.");
            }
            Axis::Up => match self {
                Axis::Left => Axis::UpLeft,
                Axis::Neutral => Axis::Up,
                Axis::Right => Axis::UpRight,
                _ => self,
            },
            Axis::Down => match self {
                Axis::Left => Axis::DownLeft,
                Axis::Neutral => Axis::Down,
                Axis::Right => Axis::DownRight,
                _ => self,
            },
            Axis::Left => match self {
                Axis::Up => Axis::UpLeft,
                Axis::Neutral => Axis::Left,
                Axis::Down => Axis::DownLeft,
                _ => self,
            },
            Axis::Right => match self {
                Axis::Up => Axis::UpRight,
                Axis::Neutral => Axis::Right,
                Axis::Down => Axis::DownRight,
                _ => self,
            },
            Axis::Neutral => self,
        }
    }
    pub fn remove(self, new: Axis) -> Self {
        match new {
            Axis::UpRight | Axis::UpLeft | Axis::DownRight | Axis::DownLeft => {
                panic!("Removing diagonal doesn't make sense.");
            }
            Axis::Up => match self {
                Axis::UpLeft => Axis::Left,
                Axis::Up => Axis::Neutral,
                Axis::UpRight => Axis::Right,
                _ => self,
            },
            Axis::Down => match self {
                Axis::DownLeft => Axis::Left,
                Axis::Down => Axis::Neutral,
                Axis::DownRight => Axis::Right,
                _ => self,
            },
            Axis::Left => match self {
                Axis::UpLeft => Axis::Up,
                Axis::Left => Axis::Neutral,
                Axis::DownLeft => Axis::Down,
                _ => self,
            },
            Axis::Right => match self {
                Axis::UpRight => Axis::Up,
                Axis::Right => Axis::Neutral,
                Axis::DownRight => Axis::Down,
                _ => self,
            },
            Axis::Neutral => self,
        }
    }
}
