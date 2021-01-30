pub mod directed_axis;
pub mod direction;
pub mod facing;

use std::ops::BitAnd;

pub use directed_axis::DirectedAxis;
pub use direction::Direction;
pub use facing::Facing;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, Display)]
pub enum Axis {
    Neutral = 0b0000,
    Up = 0b1000,
    Down = 0b0001,
    Right = 0b0010,
    Left = 0b0100,
    UpRight = 0b1010,
    UpLeft = 0b1100,
    DownRight = 0b0011,
    DownLeft = 0b0101,
}

impl BitAnd for Axis {
    type Output = bool;
    fn bitand(self, rhs: Self) -> Self::Output {
        self as u8 & rhs as u8 != 0
    }
}
/*
#[test]
fn test_axis() {
    //

    assert_eq!(TestAxis::DownRight & TestAxis::DownLeft, true);
    assert_eq!(TestAxis::DownRight & TestAxis::UpRight, true);
    assert_eq!(TestAxis::DownRight & TestAxis::UpLeft, false);

    assert_eq!(TestAxis::Right & TestAxis::DownRight, true);
    assert_eq!(TestAxis::Right & TestAxis::Left, false);
    assert_eq!(TestAxis::Right & TestAxis::Up, false);
    assert_eq!(TestAxis::Right & TestAxis::Neutral, false);
} */

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
