use crate::{axis::Axis, button::ButtonSet};

use super::button::ButtonState;
use serde::{Deserialize, Serialize};

pub type RawAxis = [i32; 2];

pub fn matches_cardinal(lhs: RawAxis, rhs: RawAxis) -> bool {
    !lhs.iter()
        .zip(rhs.iter())
        .filter(|(l, _)| **l != 0)
        .all(|(l, r)| l != r)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InputState {
    pub axis: RawAxis,
    pub buttons: [ButtonState; 5],
}

impl InputState {
    pub fn axis(&self) -> Axis {
        self.axis.into()
    }
    pub fn just_pressed(&self) -> ButtonSet {
        self.buttons
            .iter()
            .enumerate()
            .filter(|(_, state)| **state == ButtonState::JustPressed)
            .fold(ButtonSet::default(), |acc, (idx, _)| {
                acc | ButtonSet::from_id(idx)
            })
    }
    pub fn button_set(&self) -> ButtonSet {
        self.buttons
            .iter()
            .enumerate()
            .filter(|(_, state)| state.is_pressed())
            .fold(ButtonSet::default(), |acc, (idx, _)| {
                acc | ButtonSet::from_id(idx)
            })
    }
}
