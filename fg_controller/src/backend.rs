pub use fg_input::axis::Axis;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};
use strum::EnumIter;

pub struct Event<ControllerId> {
    pub id: ControllerId,
    pub event_type: EventType,
}
pub enum EventType {
    Connect,
    Disconnect,
}

pub trait ControllerBackend<'this> {
    type ControllerId;
    type ControllerIter: Iterator<Item = Self::ControllerId>;

    fn poll(&mut self);

    fn active_controller(&self) -> Option<Self::ControllerId>;

    fn active_state(&self) -> Option<(Self::ControllerId, ControllerState)> {
        let active = self.active_controller()?;
        let state = self.current_state(&active);
        Some((active, state))
    }

    fn controllers(&'this self) -> Self::ControllerIter;

    fn current_state(&self, id: &Self::ControllerId) -> ControllerState;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, EnumIter)]
pub enum Button {
    A,
    B,
    X,
    Y,
    Start,
    Back,
    Guide,
    L1,
    L2,
    LeftStick,
    R1,
    R2,
    RightStick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ControllerState {
    pub dpad: Axis,
    pub left_stick: Axis,
    pub right_stick: Axis,

    pub buttons: [bool; 13],
}

impl ControllerState {
    pub fn axis(&self) -> Axis {
        self.dpad + self.left_stick
    }
}

impl Index<Button> for ControllerState {
    type Output = bool;

    fn index(&self, index: Button) -> &Self::Output {
        &self.buttons[index as usize]
    }
}

impl IndexMut<Button> for ControllerState {
    fn index_mut(&mut self, index: Button) -> &mut Self::Output {
        &mut self.buttons[index as usize]
    }
}
