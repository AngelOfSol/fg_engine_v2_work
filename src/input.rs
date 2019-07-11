use serde::{Deserialize, Serialize};

use ringbuffer::{RingBuffer, RingBufferIter};

const MOTION_DIRECTION_SIZE: usize = 5;
const MOTION_LENGTH: usize = 5 * MOTION_DIRECTION_SIZE;
const BUFFER_LENGTH: usize = MOTION_LENGTH + MOTION_DIRECTION_SIZE;

mod control_scheme;

#[macro_use]
mod motion;
mod ringbuffer;

use std::ops::{Index, IndexMut};

pub use control_scheme::PadControlScheme;
pub use motion::{read_inputs, Direction, Input, Standing};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub struct InputState {
    pub axis: Axis,
    buttons: [ButtonState; 4],
}

impl Index<Button> for InputState {
    type Output = ButtonState;
    fn index(&self, idx: Button) -> &Self::Output {
        &self.buttons[idx as usize]
    }
}
impl IndexMut<Button> for InputState {
    fn index_mut(&mut self, idx: Button) -> &mut Self::Output {
        &mut self.buttons[idx as usize]
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ButtonState {
    Released,
    JustReleased,
    Pressed,
    JustPressed,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(usize)]
pub enum Button {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl Button {
    pub fn from_usize(value: usize) -> Self {
        match value {
            0 => Button::A,
            1 => Button::B,
            2 => Button::C,
            3 => Button::D,
            _ => panic!("invalid button value"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Axis {
    Up,
    Down,
    Right,
    Left,
    Neutral,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Axis {
    pub fn is_horizontal(self) -> bool {
        match self {
            Axis::Left | Axis::Right => true,
            _ => false,
        }
    }
    pub fn is_command(self) -> bool {
        !self.is_neutral()
    }
    pub fn is_neutral(self) -> bool {
        match self {
            Axis::Up | Axis::Neutral | Axis::Down => true,
            _ => false,
        }
    }

    pub fn get_direction(self) -> Option<Direction> {
        match self {
            Axis::UpLeft | Axis::Left | Axis::DownLeft => Some(Direction::Backward),
            Axis::UpRight | Axis::Right | Axis::DownRight => Some(Direction::Forward),
            _ => None,
        }
    }
    pub fn get_standing(self) -> Standing {
        match self {
            Axis::DownRight | Axis::Down | Axis::DownLeft => Standing::Crouching,
            _ => Standing::Standing,
        }
    }

    pub fn add(self, new: Axis) -> Self {
        match new {
            Axis::UpRight | Axis::UpLeft | Axis::DownRight | Axis::DownLeft => {
                panic!("Adding diagonal doesn't make sense.");
            }
            Axis::Up => match self {
                Axis::Left => Axis::UpLeft,
                Axis::Neutral => Axis::Up,
                Axis::Right => Axis::UpRight,
                _ => self,
            },
            Axis::Down => match self {
                Axis::Left => Axis::DownLeft,
                Axis::Neutral => Axis::Down,
                Axis::Right => Axis::DownRight,
                _ => self,
            },
            Axis::Left => match self {
                Axis::Up => Axis::UpLeft,
                Axis::Neutral => Axis::Left,
                Axis::Down => Axis::DownLeft,
                _ => self,
            },
            Axis::Right => match self {
                Axis::Up => Axis::UpRight,
                Axis::Neutral => Axis::Right,
                Axis::Down => Axis::DownRight,
                _ => self,
            },
            Axis::Neutral => self,
        }
    }
    pub fn remove(self, new: Axis) -> Self {
        match new {
            Axis::UpRight | Axis::UpLeft | Axis::DownRight | Axis::DownLeft => {
                panic!("Removing diagonal doesn't make sense.");
            }
            Axis::Up => match self {
                Axis::UpLeft => Axis::Left,
                Axis::Up => Axis::Neutral,
                Axis::UpRight => Axis::Right,
                _ => self,
            },
            Axis::Down => match self {
                Axis::DownLeft => Axis::Left,
                Axis::Down => Axis::Neutral,
                Axis::DownRight => Axis::Right,
                _ => self,
            },
            Axis::Left => match self {
                Axis::UpLeft => Axis::Up,
                Axis::Left => Axis::Neutral,
                Axis::DownLeft => Axis::Down,
                _ => self,
            },
            Axis::Right => match self {
                Axis::UpRight => Axis::Up,
                Axis::Right => Axis::Neutral,
                Axis::DownRight => Axis::Down,
                _ => self,
            },
            Axis::Neutral => self,
        }
    }
}

pub struct InputBuffer {
    buffer: RingBuffer,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            buffer: RingBuffer::new(),
        }
    }

    pub fn top(&self) -> &InputState {
        self.buffer.top()
    }

    pub fn iter(&self) -> RingBufferIter<'_> {
        self.buffer.iter()
    }

    pub fn push(&mut self, input: InputState) {
        self.buffer.push(input);
    }
}
