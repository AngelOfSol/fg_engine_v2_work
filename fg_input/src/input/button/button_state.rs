use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, Display)]
pub enum ButtonState {
    Released,
    JustReleased,
    Pressed,
    JustPressed,
}

impl ButtonState {
    pub fn is_pressed(self) -> bool {
        matches!(self, ButtonState::Pressed | ButtonState::JustPressed)
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::Released
    }
}
