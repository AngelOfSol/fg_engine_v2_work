use super::{Axis, Facing};
use crate::character::components::Guard;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, Inspect, PartialOrd, Ord,
)]
pub enum DirectedAxis {
    DownBackward,
    Down,
    DownForward,
    Backward,
    Neutral,
    Forward,
    UpBackward,
    Up,
    UpForward,
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
    pub fn direction_multiplier(self, facing: bool) -> i32 {
        let facing = if facing { 1 } else { -1 };
        let self_value = match self {
            DirectedAxis::Forward | DirectedAxis::UpForward | DirectedAxis::DownForward => 1,
            DirectedAxis::Backward | DirectedAxis::UpBackward | DirectedAxis::DownBackward => -1,
            _ => 0,
        };
        facing * self_value
    }

    pub fn is_cardinal(self) -> bool {
        matches!(
            self,
            DirectedAxis::Forward | DirectedAxis::Up | DirectedAxis::Backward | DirectedAxis::Down
        )
    }
    pub fn matches_cardinal(self, target: DirectedAxis) -> bool {
        target.is_cardinal()
            && match target {
                DirectedAxis::Forward => match self {
                    DirectedAxis::UpForward | DirectedAxis::DownForward | DirectedAxis::Forward => {
                        true
                    }
                    _ => false,
                },
                DirectedAxis::Up => matches!(
                    self,
                    DirectedAxis::UpForward | DirectedAxis::UpBackward | DirectedAxis::Up
                ),
                DirectedAxis::Backward => matches!(
                    self,
                    DirectedAxis::UpBackward | DirectedAxis::DownBackward | DirectedAxis::Backward
                ),
                DirectedAxis::Down => match self {
                    DirectedAxis::DownForward | DirectedAxis::DownBackward | DirectedAxis::Down => {
                        true
                    }
                    _ => false,
                },
                _ => unreachable!(),
            }
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
            DirectedAxis::Forward => DirectedAxis::Backward,
            DirectedAxis::DownForward => DirectedAxis::DownBackward,
            DirectedAxis::UpForward => DirectedAxis::UpBackward,
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

    pub fn is_horizontal(self) -> bool {
        !matches!(
            self,
            DirectedAxis::Up | DirectedAxis::Neutral | DirectedAxis::Down
        )
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
