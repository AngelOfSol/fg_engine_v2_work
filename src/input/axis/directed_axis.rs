use super::{Axis, Facing};
use crate::character::components::Guard;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DirectedAxis {
    Up,
    Down,
    Forward,
    Backward,
    Neutral,
    UpForward,
    UpBackward,
    DownForward,
    DownBackward,
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
        match self {
            DirectedAxis::Forward
            | DirectedAxis::Up
            | DirectedAxis::Backward
            | DirectedAxis::Down => true,
            _ => false,
        }
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
                DirectedAxis::Up => match self {
                    DirectedAxis::UpForward | DirectedAxis::UpBackward | DirectedAxis::Up => true,
                    _ => false,
                },
                DirectedAxis::Backward => match self {
                    DirectedAxis::UpBackward
                    | DirectedAxis::DownBackward
                    | DirectedAxis::Backward => true,
                    _ => false,
                },
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
        match self {
            DirectedAxis::Backward | DirectedAxis::UpBackward | DirectedAxis::DownBackward => true,
            _ => false,
        }
    }
    pub fn is_down(self) -> bool {
        match self {
            DirectedAxis::Down | DirectedAxis::DownBackward | DirectedAxis::DownForward => true,
            _ => false,
        }
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
        match self {
            DirectedAxis::Up | DirectedAxis::Neutral | DirectedAxis::Down => false,
            _ => true,
        }
    }

    pub fn is_blocking(self, guard: Guard) -> bool {
        match guard {
            Guard::Mid => true,
            Guard::High => !self.is_down(),
            Guard::Low => self.is_down(),
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
