use crate::{axis::Axis, button::ButtonSet};

use super::button::ButtonState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputState {
    pub axis: [i32; 2],
    pub buttons: [ButtonState; 5],
}

impl InputState {
    pub fn axis(&self) -> Axis {
        self.axis.into()
    }
    pub fn button_set(&self) -> ButtonSet {
        self.buttons
            .iter()
            .enumerate()
            .filter(|(_, state)| state.is_pressed())
            .fold(ButtonSet::default(), |acc, (idx, _)| {
                acc | ButtonSet::from_id(idx + 1)
            })
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            axis: [0, 0],
            buttons: [
                ButtonState::Released,
                ButtonState::Released,
                ButtonState::Released,
                ButtonState::Released,
                ButtonState::Released,
            ],
        }
    }
}
