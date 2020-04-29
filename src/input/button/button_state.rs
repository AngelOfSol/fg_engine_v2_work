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
    pub fn into_bits(self) -> u16 {
        match self {
            ButtonState::Released => 0b00,
            ButtonState::JustReleased => 0b10,
            ButtonState::Pressed => 0b01,
            ButtonState::JustPressed => 0b11,
        }
    }

    pub fn from_bits(value: u16) -> Option<Self> {
        match value {
            0b00 => Some(ButtonState::Released),
            0b10 => Some(ButtonState::JustReleased),
            0b01 => Some(ButtonState::Pressed),
            0b11 => Some(ButtonState::JustPressed),
            _ => None,
        }
    }
}
