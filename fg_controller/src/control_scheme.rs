use crate::pads_context::GamepadId;

use std::collections::{HashMap, HashSet};

use fg_input::{button::ButtonState, InputState};
use sdl2::controller::Button as SdlButton;

pub type PadControlScheme = ControlScheme<SdlButton>;

#[derive(Clone, Debug)]
pub struct ControlScheme<ButtonCode> {
    axis: HashMap<ButtonCode, [i32; 2]>,
    pub buttons: [HashSet<ButtonCode>; 5],
    pub gamepad: GamepadId,
}

pub fn is_valid_input_button(button: SdlButton) -> bool {
    matches!(
        button,
        SdlButton::A
            | SdlButton::B
            | SdlButton::X
            | SdlButton::Y
            | SdlButton::LeftShoulder
            | SdlButton::RightShoulder
    )
}

pub fn render_button_list(list: &HashSet<SdlButton>) -> String {
    // TODO use fold instead
    let mut ret = "".to_owned();

    for value in list.iter() {
        let string_value = match value {
            SdlButton::A => "A",
            SdlButton::B => "B",
            SdlButton::X => "X",
            SdlButton::Y => "Y",
            SdlButton::LeftShoulder => "L1",
            SdlButton::RightShoulder => "R1",
            _ => "invalid",
        };
        if ret.is_empty() {
            ret = string_value.to_owned();
        } else {
            ret = format!("{}, {}", ret, string_value);
        }
    }

    ret
}

impl ControlScheme<SdlButton> {
    pub fn new(id: GamepadId) -> Self {
        let mut ret = Self {
            axis: HashMap::new(),
            buttons: [
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
            gamepad: id,
        };

        ret.buttons[0].insert(SdlButton::X);
        ret.buttons[1].insert(SdlButton::Y);
        ret.buttons[2].insert(SdlButton::B);
        ret.buttons[3].insert(SdlButton::A);
        ret.buttons[4].insert(SdlButton::RightShoulder);

        ret.axis.insert(SdlButton::DPadUp, [0, 1]);
        ret.axis.insert(SdlButton::DPadDown, [0, -1]);
        ret.axis.insert(SdlButton::DPadRight, [1, 0]);
        ret.axis.insert(SdlButton::DPadLeft, [-1, 0]);

        ret
    }

    pub fn update_frame(&self, input: &mut InputState) {
        for state in input.buttons.iter_mut() {
            *state = match &state {
                ButtonState::JustPressed => ButtonState::Pressed,
                ButtonState::JustReleased => ButtonState::Released,
                value => **value,
            }
        }
    }

    pub fn handle_press(&self, button: SdlButton, input: &mut InputState) {
        if self.axis.contains_key(&button) {
            for (old, modify) in input.axis.iter_mut().zip(self.axis[&button].iter()) {
                *old += modify;
            }
        }

        for (set, state) in self.buttons.iter().zip(input.buttons.iter_mut()) {
            if set.contains(&button) {
                *state = ButtonState::JustPressed;
            }
        }
    }

    pub fn handle_release(&self, button: SdlButton, input: &mut InputState) {
        if self.axis.contains_key(&button) {
            for (old, modify) in input.axis.iter_mut().zip(self.axis[&button].iter()) {
                *old -= modify;
            }
        }

        for (set, state) in self.buttons.iter().zip(input.buttons.iter_mut()) {
            if set.contains(&button) {
                *state = ButtonState::JustReleased;
            }
        }
    }
}
