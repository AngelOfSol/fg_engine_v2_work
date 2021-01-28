use super::button::{Button, ButtonState};
use super::Axis;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputState {
    pub axis: Axis,
    pub buttons: [ButtonState; 5],
}

impl std::fmt::Display for InputState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Axis: {}, A: {}, B: {}, C: {}, D: {}, E: {})",
            self.axis,
            self.buttons[0],
            self.buttons[1],
            self.buttons[2],
            self.buttons[3],
            self.buttons[4]
        )
    }
}
impl Index<Button> for InputState {
    type Output = ButtonState;
    fn index(&self, idx: Button) -> &Self::Output {
        &self.buttons[idx.as_id()]
    }
}
impl IndexMut<Button> for InputState {
    fn index_mut(&mut self, idx: Button) -> &mut Self::Output {
        &mut self.buttons[idx.as_id()]
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            axis: Axis::Neutral,
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
