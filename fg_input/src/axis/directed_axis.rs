use super::{Axis, Facing};
use crate::guard::Guard;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, Inspect, PartialOrd, Ord,
)]
pub enum DirectedAxis {
    DownBackward,
    DownForward,
    Down,
    UpBackward,
    UpForward,
    Up,
    Backward,
    Forward,
    Neutral,
}

impl Display for DirectedAxis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DownBackward => write!(f, "1"),
            Self::Down => write!(f, "2"),
            Self::DownForward => write!(f, "3"),
            Self::Backward => write!(f, "4"),
            Self::Neutral => write!(f, "5"),
            Self::Forward => write!(f, "6"),
            Self::UpBackward => write!(f, "7"),
            Self::Up => write!(f, "8"),
            Self::UpForward => write!(f, "9"),
        }
    }
}

impl Default for DirectedAxis {
    fn default() -> Self {
        Self::Neutral
    }
}

impl DirectedAxis {
    pub fn matches(self, rhs: Self) -> bool {
        matches!(
            (self, rhs),
            (
                DirectedAxis::DownForward,
                DirectedAxis::DownForward | DirectedAxis::Down
            ) | (
                DirectedAxis::UpForward,
                DirectedAxis::UpForward | DirectedAxis::Up
            ) | (
                DirectedAxis::DownBackward,
                DirectedAxis::DownBackward | DirectedAxis::Down
            ) | (
                DirectedAxis::UpBackward,
                DirectedAxis::UpBackward | DirectedAxis::Up
            ) | (
                DirectedAxis::Forward,
                DirectedAxis::Forward | DirectedAxis::Neutral
            ) | (
                DirectedAxis::Backward,
                DirectedAxis::Backward | DirectedAxis::Neutral
            ) | (DirectedAxis::Down, DirectedAxis::Down)
                | (DirectedAxis::Up, DirectedAxis::Up)
                | (_, DirectedAxis::Neutral)
        )
    }

    pub fn is_backward(self) -> bool {
        matches!(
            self,
            DirectedAxis::Backward | DirectedAxis::UpBackward | DirectedAxis::DownBackward
        )
    }
    pub fn is_forward(self) -> bool {
        matches!(
            self,
            DirectedAxis::Forward | DirectedAxis::UpForward | DirectedAxis::DownForward
        )
    }
    pub fn is_down(self) -> bool {
        matches!(
            self,
            DirectedAxis::Down | DirectedAxis::DownBackward | DirectedAxis::DownForward
        )
    }

    pub fn invert(self) -> Self {
        match self {
            // Forward to Backward
            DirectedAxis::Forward => DirectedAxis::Backward,
            DirectedAxis::DownForward => DirectedAxis::DownBackward,
            DirectedAxis::UpForward => DirectedAxis::UpBackward,
            // Backward to Forward
            DirectedAxis::Backward => DirectedAxis::Forward,
            DirectedAxis::DownBackward => DirectedAxis::DownForward,
            DirectedAxis::UpBackward => DirectedAxis::UpForward,
            value => value,
        }
    }
    pub fn from_facing(item: Axis, facing: Facing) -> Self {
        let ret = match item {
            Axis::Up => DirectedAxis::Up,
            Axis::Down => DirectedAxis::Down,
            Axis::Right => DirectedAxis::Forward,
            Axis::Left => DirectedAxis::Backward,
            Axis::Neutral => DirectedAxis::Neutral,
            Axis::UpRight => DirectedAxis::UpForward,
            Axis::UpLeft => DirectedAxis::UpBackward,
            Axis::DownRight => DirectedAxis::DownForward,
            Axis::DownLeft => DirectedAxis::DownBackward,
        };

        if facing == Facing::Left {
            ret.invert()
        } else {
            ret
        }
    }

    pub fn is_guarding(self, guard: Guard) -> bool {
        match guard {
            Guard::Mid => true,
            Guard::High => !self.is_down(),
            Guard::Low => self.is_down(),
        }
    }

    pub fn is_blocking(self, crossup: bool) -> bool {
        if !crossup {
            self.is_backward()
        } else {
            self.is_forward()
        }
    }
}

impl From<Axis> for DirectedAxis {
    fn from(item: Axis) -> Self {
        match item {
            Axis::Up => DirectedAxis::Up,
            Axis::Down => DirectedAxis::Down,
            Axis::Right => DirectedAxis::Forward,
            Axis::Left => DirectedAxis::Backward,
            Axis::Neutral => DirectedAxis::Neutral,
            Axis::UpRight => DirectedAxis::UpForward,
            Axis::UpLeft => DirectedAxis::UpBackward,
            Axis::DownRight => DirectedAxis::DownForward,
            Axis::DownLeft => DirectedAxis::DownBackward,
        }
    }
}
