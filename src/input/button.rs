mod button_set;
mod button_state;

pub use button_set::ButtonSet;
pub use button_state::ButtonState;

use serde::{Deserialize, Serialize};
use std::ops::BitOr;
use strum::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, Display)]
#[repr(u8)]
pub enum Button {
    A = 0b00001,
    B = 0b00010,
    C = 0b00100,
    D = 0b01000,
    E = 0b10000,
}

impl Button {
    pub fn from_id(value: usize) -> Self {
        match value {
            0 => Button::A,
            1 => Button::B,
            2 => Button::C,
            3 => Button::D,
            4 => Button::E,
            _ => panic!("invalid button value"),
        }
    }
    pub fn as_id(self) -> usize {
        match self {
            Button::A => 0,
            Button::B => 1,
            Button::C => 2,
            Button::D => 3,
            Button::E => 4,
        }
    }
}

impl<Rhs> BitOr<Rhs> for Button
where
    Rhs: Into<ButtonSet>,
{
    type Output = ButtonSet;
    fn bitor(self, rhs: Rhs) -> Self::Output {
        ButtonSet::from(self) | rhs.into()
    }
}
