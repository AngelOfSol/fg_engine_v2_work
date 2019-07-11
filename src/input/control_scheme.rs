use super::{Axis, Button, ButtonState, InputState};

use std::collections::{HashMap, HashSet};

use gilrs::ev::Button as GilButton;

pub type PadControlScheme = ControlScheme<GilButton>;

pub struct ControlScheme<ButtonCode> {
    axis: HashMap<ButtonCode, Axis>,
    buttons: [HashSet<ButtonCode>; 4],
}

impl ControlScheme<GilButton> {
    pub fn new() -> Self {
        let mut ret = Self {
            axis: HashMap::new(),
            buttons: [
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
        };

        ret.buttons[Button::A as usize].insert(GilButton::West);
        ret.buttons[Button::A as usize].insert(GilButton::RightTrigger);
        ret.buttons[Button::B as usize].insert(GilButton::North);
        ret.buttons[Button::B as usize].insert(GilButton::RightTrigger);
        ret.buttons[Button::C as usize].insert(GilButton::East);
        ret.buttons[Button::D as usize].insert(GilButton::South);

        ret.axis.insert(GilButton::DPadUp, Axis::Up);
        ret.axis.insert(GilButton::DPadDown, Axis::Down);
        ret.axis.insert(GilButton::DPadRight, Axis::Right);
        ret.axis.insert(GilButton::DPadLeft, Axis::Left);

        ret
    }

    pub fn update_frame(&self, mut input: InputState) -> InputState {
        for state in input.buttons.iter_mut() {
            *state = match &state {
                ButtonState::JustPressed => ButtonState::Pressed,
                ButtonState::JustReleased => ButtonState::Released,
                value => **value,
            }
        }
        input
    }

    pub fn handle_press(&self, button: GilButton, mut input: InputState) -> InputState {
        if self.axis.contains_key(&button) {
            input.axis = input.axis.add(self.axis[&button]);
        }

        for (set, state) in self.buttons.iter().zip(input.buttons.iter_mut()) {
            if set.contains(&button) {
                *state = ButtonState::JustPressed;
            }
        }

        input
    }

    pub fn handle_release(&self, button: GilButton, mut input: InputState) -> InputState {
        if self.axis.contains_key(&button) {
            input.axis = input.axis.remove(self.axis[&button]);
        }

        for (set, state) in self.buttons.iter().zip(input.buttons.iter_mut()) {
            if set.contains(&button) {
                *state = ButtonState::JustReleased;
            }
        }

        input
    }
}
