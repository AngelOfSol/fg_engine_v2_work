use super::button::{Button, ButtonState};
use super::Axis;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct InputState {
    pub axis: Axis,
    pub buttons: [ButtonState; 4],
}

impl std::fmt::Display for InputState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Axis: {}, A: {}, B: {}, C: {}, D: {})",
            self.axis, self.buttons[0], self.buttons[1], self.buttons[2], self.buttons[3],
        )
    }
}

impl Serialize for InputState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (
            (self.buttons[0].into_bits()
                + (self.buttons[1].into_bits() << 2)
                + (self.buttons[2].into_bits() << 4)
                + (self.buttons[3].into_bits() << 6)),
            self.axis.into_bits(),
        )
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for InputState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (abcd, axis) = <(u8, u8)>::deserialize(deserializer)?;

        Ok(InputState {
            buttons: [
                ButtonState::from_bits(abcd & 0b00000011).unwrap(),
                ButtonState::from_bits((abcd >> 2) & 0b00000011).unwrap(),
                ButtonState::from_bits((abcd >> 4) & 0b00000011).unwrap(),
                ButtonState::from_bits((abcd >> 6) & 0b00000011).unwrap(),
            ],
            axis: Axis::from_bits(axis).ok_or(serde::de::Error::invalid_value(
                serde::de::Unexpected::Other("u8"),
                &"u4",
            ))?,
        })
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
            ],
        }
    }
}
