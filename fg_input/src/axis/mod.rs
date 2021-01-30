pub mod directed_axis;
pub mod direction;
pub mod facing;

use std::{
    convert::{TryFrom, TryInto},
    ops::{Add, AddAssign, BitAnd, Neg, Sub, SubAssign},
};

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

impl Default for Axis {
    fn default() -> Self {
        Self::Neutral
    }
}

impl Axis {
    pub fn x(self) -> Self {
        (self as u8 & 0b0110).try_into().unwrap()
    }

    pub fn y(self) -> Self {
        (self as u8 & 0b1001).try_into().unwrap()
    }

    fn socd(lhs: Self, rhs: Self) -> Self {
        let x = lhs.x() + rhs.x();
        let y = if lhs.y() == Axis::Up || rhs.y() == Axis::Up {
            Axis::Up
        } else {
            lhs + rhs
        };

        x + y
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvalidAxisValue;
impl TryFrom<u8> for Axis {
    type Error = InvalidAxisValue;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0000 | 0b0110 | 0b1001 | 0b1111 => Ok(Axis::Neutral),
            0b1000 | 0b1110 => Ok(Axis::Up),
            0b0001 | 0b0111 => Ok(Axis::Down),
            0b0010 | 0b1011 => Ok(Axis::Right),
            0b0100 | 0b1101 => Ok(Axis::Left),
            0b1010 => Ok(Axis::UpRight),
            0b1100 => Ok(Axis::UpLeft),
            0b0011 => Ok(Axis::DownRight),
            0b0101 => Ok(Axis::DownLeft),
            _ => Err(InvalidAxisValue),
        }
    }
}

impl BitAnd for Axis {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        (self as u8 & rhs as u8).try_into().unwrap()
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add for Axis {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        (self as u8 | rhs as u8).try_into().unwrap()
    }
}

impl AddAssign for Axis {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Axis {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl SubAssign for Axis {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for Axis {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Axis::Neutral => Axis::Neutral,
            Axis::Up => Axis::Down,
            Axis::Down => Axis::Up,
            Axis::Right => Axis::Left,
            Axis::Left => Axis::Right,
            Axis::UpLeft => Axis::DownRight,
            Axis::DownRight => Axis::UpLeft,
            Axis::UpRight => Axis::DownLeft,
            Axis::DownLeft => Axis::UpRight,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Axis;

    #[test]
    fn test_bitand() {
        assert_eq!(Axis::DownRight & Axis::DownLeft, Axis::Down);
        assert_eq!(Axis::DownRight & Axis::UpRight, Axis::Right);
        assert_eq!(Axis::DownRight & Axis::UpLeft, Axis::Neutral);

        assert_eq!(Axis::Right & Axis::DownRight, Axis::Right);
        assert_eq!(Axis::Right & Axis::Left, Axis::Neutral);
        assert_eq!(Axis::Right & Axis::Up, Axis::Neutral);
        assert_eq!(Axis::Right & Axis::Neutral, Axis::Neutral);
    }

    #[test]
    fn test_add() {
        assert_eq!(Axis::DownRight + Axis::DownLeft, Axis::Down);
        assert_eq!(Axis::DownRight + Axis::UpRight, Axis::Right);
        assert_eq!(Axis::DownRight + Axis::UpLeft, Axis::Neutral);

        assert_eq!(Axis::Right + Axis::DownRight, Axis::DownRight);
        assert_eq!(Axis::Right + Axis::Left, Axis::Neutral);
        assert_eq!(Axis::Right + Axis::Up, Axis::UpRight);
        assert_eq!(Axis::Right + Axis::Neutral, Axis::Right);

        assert_eq!(Axis::Neutral + Axis::Up, Axis::Up);
        assert_eq!(Axis::Neutral + Axis::Down, Axis::Down);
        assert_eq!(Axis::Neutral + Axis::Right, Axis::Right);
        assert_eq!(Axis::Neutral + Axis::Left, Axis::Left);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn test_sub() {
        assert_eq!(Axis::DownRight - Axis::DownLeft, Axis::Right);
        assert_eq!(Axis::DownRight - Axis::UpRight, Axis::Down);
        assert_eq!(Axis::DownRight - Axis::UpLeft, Axis::DownRight);

        assert_eq!(Axis::Right - Axis::DownRight, Axis::Up);
        assert_eq!(Axis::Right - Axis::Left, Axis::Right);
        assert_eq!(Axis::UpRight - Axis::Up, Axis::Right);
        assert_eq!(Axis::UpRight - Axis::Right, Axis::Up);
        assert_eq!(Axis::Right - Axis::Up, Axis::DownRight);
        assert_eq!(Axis::Right - Axis::Neutral, Axis::Right);

        assert_eq!(Axis::Left - Axis::Left, Axis::Neutral);

        assert_eq!(Axis::Neutral - Axis::Up, Axis::Down);
        assert_eq!(Axis::Neutral - Axis::Down, Axis::Up);
        assert_eq!(Axis::Neutral - Axis::Right, Axis::Left);
        assert_eq!(Axis::Neutral - Axis::Left, Axis::Right);
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Axis::Neutral, Axis::Neutral);
        assert_eq!(-Axis::Up, Axis::Down);
        assert_eq!(-Axis::Down, Axis::Up);
        assert_eq!(-Axis::Right, Axis::Left);
        assert_eq!(-Axis::Left, Axis::Right);
        assert_eq!(-Axis::UpLeft, Axis::DownRight);
        assert_eq!(-Axis::DownRight, Axis::UpLeft);
        assert_eq!(-Axis::UpRight, Axis::DownLeft);
        assert_eq!(-Axis::DownLeft, Axis::UpRight);
    }

    #[test]
    fn test_socd() {
        assert_eq!(Axis::socd(Axis::Left, Axis::Right), Axis::Neutral);
        assert_eq!(Axis::socd(Axis::Right, Axis::Left), Axis::Neutral);

        assert_eq!(Axis::socd(Axis::Down, Axis::Up), Axis::Up);
        assert_eq!(Axis::socd(Axis::Up, Axis::Down), Axis::Up);

        assert_eq!(Axis::socd(Axis::DownLeft, Axis::UpRight), Axis::Up);
        assert_eq!(Axis::socd(Axis::UpRight, Axis::DownLeft,), Axis::Up);

        assert_eq!(Axis::socd(Axis::Left, Axis::Down), Axis::DownLeft);
        assert_eq!(Axis::socd(Axis::Down, Axis::Left), Axis::DownLeft);

        assert_eq!(Axis::socd(Axis::DownLeft, Axis::Neutral), Axis::DownLeft);
        assert_eq!(Axis::socd(Axis::Neutral, Axis::DownLeft,), Axis::DownLeft);

        assert_eq!(Axis::socd(Axis::UpRight, Axis::Neutral), Axis::UpRight);
        assert_eq!(Axis::socd(Axis::Neutral, Axis::UpRight), Axis::UpRight);
    }
}
