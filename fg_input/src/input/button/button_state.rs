use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, Display)]
pub enum ButtonState {
    Released = 0b00,
    JustReleased = 0b10,
    Pressed = 0b01,
    JustPressed = 0b11,
}

impl ButtonState {
    pub fn is_pressed(self) -> bool {
        matches!(self, ButtonState::Pressed | ButtonState::JustPressed)
    }

    pub fn next_with(self, new: bool) -> Self {
        match (self.is_pressed(), new) {
            (false, false) => Self::Released,
            (true, true) => Self::Pressed,
            (true, false) => Self::JustReleased,
            (false, true) => Self::JustPressed,
        }
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::Released
    }
}
