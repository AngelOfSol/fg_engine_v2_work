use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, Display)]
pub enum ButtonState {
    Released,
    JustReleased,
    Pressed,
    JustPressed,
}

impl ButtonState {
    pub fn is_pressed(self) -> bool {
        match self {
            ButtonState::Pressed | ButtonState::JustPressed => true,
            _ => false,
        }
    }
}
