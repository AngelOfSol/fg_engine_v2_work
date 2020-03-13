pub mod control_scheme;
#[macro_use]
mod motion;
mod ringbuffer;

use crate::typedefs::{collision, graphics};
pub use motion::{read_inputs, DirectedAxis, Direction, Input};
use ringbuffer::{RingBuffer, RingBufferIter};
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

const MOTION_DIRECTION_SIZE: usize = 10;
const BUFFER_LENGTH: usize = 6 * MOTION_DIRECTION_SIZE;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub struct InputState {
    pub axis: Axis,
    buttons: [ButtonState; 4],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Facing {
    Left,
    Right,
}

impl Facing {
    pub fn fix_rotation(self, value: f32) -> f32 {
        match self {
            Facing::Left => {
                std::f32::consts::PI / 2.0
                    - value.signum() * (std::f32::consts::PI / 2.0 - value).abs()
            }
            Facing::Right => value,
        }
    }
    pub fn invert(self) -> Self {
        match self {
            Facing::Left => Facing::Right,
            Facing::Right => Facing::Left,
        }
    }

    pub fn fix_graphics(self, data: graphics::Vec2) -> graphics::Vec2 {
        data.component_mul(&self.graphics_multiplier())
    }
    pub fn graphics_multiplier(self) -> graphics::Vec2 {
        graphics::Vec2::new(
            match self {
                Facing::Left => -1.0,
                Facing::Right => 1.0,
            },
            1.0,
        )
    }
    pub fn fix_collision(self, data: collision::Vec2) -> collision::Vec2 {
        data.component_mul(&self.collision_multiplier())
    }
    pub fn collision_multiplier(self) -> collision::Vec2 {
        collision::Vec2::new(
            match self {
                Facing::Left => -1,
                Facing::Right => 1,
            },
            1,
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

impl ButtonState {
    pub fn is_pressed(self) -> bool {
        match self {
            ButtonState::Pressed | ButtonState::JustPressed => true,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ButtonSet(u8);

impl From<Button> for ButtonSet {
    fn from(value: Button) -> ButtonSet {
        ButtonSet(value as u8)
    }
}

use std::ops::{BitOr, BitOrAssign};

impl<Rhs> BitOr<Rhs> for Button
where
    Rhs: Into<ButtonSet>,
{
    type Output = ButtonSet;
    fn bitor(self, rhs: Rhs) -> Self::Output {
        ButtonSet::from(self) | rhs.into()
    }
}

impl<Rhs> BitOr<Rhs> for ButtonSet
where
    Rhs: Into<ButtonSet>,
{
    type Output = ButtonSet;
    fn bitor(self, rhs: Rhs) -> Self::Output {
        ButtonSet(self.0 | rhs.into().0)
    }
}

impl<Rhs> BitOrAssign<Rhs> for ButtonSet
where
    Rhs: Into<ButtonSet>,
{
    fn bitor_assign(&mut self, rhs: Rhs) {
        self.0 |= rhs.into().0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Button {
    A = 0b0001,
    B = 0b0010,
    C = 0b0100,
    D = 0b1000,
}

impl Button {
    pub fn from_id(value: usize) -> Self {
        match value {
            0 => Button::A,
            1 => Button::B,
            2 => Button::C,
            3 => Button::D,
            _ => panic!("invalid button value"),
        }
    }
    pub fn as_id(self) -> usize {
        match self {
            Button::A => 0,
            Button::B => 1,
            Button::C => 2,
            Button::D => 3,
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

#[derive(Clone)]
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
    pub fn top_mut(&mut self) -> &mut InputState {
        self.buffer.top_mut()
    }

    pub fn iter(&self) -> RingBufferIter<'_> {
        self.buffer.iter()
    }

    pub fn push(&mut self, input: InputState) {
        self.buffer.push(input);
    }
}
