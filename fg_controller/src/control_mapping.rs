use crate::backend::{Button, ControllerState};
use fg_input::{axis::Axis, button::ButtonSet, InputState};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Default)]
pub struct ControlMapping {
    pub buttons: HashMap<Button, ButtonSet>,
}

impl ControlMapping {
    pub fn map(&self, mut old: InputState, controller: &ControllerState) -> InputState {
        let buttons = self
            .buttons
            .iter()
            .filter(|(button, _)| controller[**button])
            .fold(ButtonSet::default(), |acc, (_, set)| acc | *set);

        for (old, (_, new)) in old.buttons.iter_mut().zip(buttons.iter()) {
            *old = old.next_with(new);
        }

        old.axis = Axis::socd(controller.dpad, controller.left_stick);
        old
    }
}
