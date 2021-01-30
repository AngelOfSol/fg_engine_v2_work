use super::button::ButtonState;
use crate::{axis::Axis, button::ButtonSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InputState {
    pub axis: Axis,
    pub buttons: [ButtonState; 8],
}

impl InputState {
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
