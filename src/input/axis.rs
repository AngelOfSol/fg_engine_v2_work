mod directed_axis;
mod direction;
mod facing;

pub use directed_axis::DirectedAxis;
pub use direction::Direction;
pub use facing::Facing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
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
    pub fn into_bits(self) -> u8 {
        match self {
            Axis::Up => 0b0000,
            Axis::Down => 0b0001,
            Axis::Right => 0b0010,
            Axis::Left => 0b0011,
            Axis::Neutral => 0b0100,
            Axis::UpRight => 0b0101,
            Axis::UpLeft => 0b0110,
            Axis::DownRight => 0b0111,
            Axis::DownLeft => 0b1000,
        }
    }

    pub fn from_bits(value: u8) -> Option<Axis> {
        match value {
            0b0000 => Some(Axis::Up),
            0b0001 => Some(Axis::Down),
            0b0010 => Some(Axis::Right),
            0b0011 => Some(Axis::Left),
            0b0100 => Some(Axis::Neutral),
            0b0101 => Some(Axis::UpRight),
            0b0110 => Some(Axis::UpLeft),
            0b0111 => Some(Axis::DownRight),
            0b1000 => Some(Axis::DownLeft),
            _ => None,
        }
    }

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
