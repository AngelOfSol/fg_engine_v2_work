use super::button::{Button, ButtonState};
use super::{Axis, InputState};
use gilrs::ev::Button as GilButton;
use gilrs::GamepadId;
use std::collections::{HashMap, HashSet};

pub type PadControlScheme = ControlScheme<GilButton>;

#[derive(Clone, Debug)]
pub struct ControlScheme<ButtonCode> {
    axis: HashMap<ButtonCode, Axis>,
    pub buttons: [HashSet<ButtonCode>; 4],
    pub gamepad: GamepadId,
}

pub fn is_valid_input_button(button: GilButton) -> bool {
    match button {
        GilButton::South
        | GilButton::East
        | GilButton::West
        | GilButton::North
        | GilButton::RightTrigger
        | GilButton::RightTrigger2
        | GilButton::LeftTrigger
        | GilButton::LeftTrigger2 => true,
        _ => false,
    }
}

pub fn render_button_list(list: &HashSet<GilButton>) -> String {
    let mut ret = "".to_owned();

    for value in list.iter() {
        let string_value = match value {
            GilButton::South => "A",
            GilButton::East => "B",
            GilButton::West => "X",
            GilButton::North => "Y",
            GilButton::RightTrigger => "R1",
            GilButton::RightTrigger2 => "R2",
            GilButton::LeftTrigger => "L1",
            GilButton::LeftTrigger2 => "L2",
            _ => "invalid",
        };
        if ret == "" {
            ret = string_value.to_owned();
        } else {
            ret = format!("{}, {}", ret, string_value);
        }
    }

    ret
}

impl ControlScheme<GilButton> {
    pub fn new(id: GamepadId) -> Self {
        let mut ret = Self {
            axis: HashMap::new(),
            buttons: [
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
            gamepad: id,
        };

        ret.buttons[Button::A.as_id()].insert(GilButton::West);
        ret.buttons[Button::A.as_id()].insert(GilButton::RightTrigger);
        ret.buttons[Button::B.as_id()].insert(GilButton::North);
        ret.buttons[Button::B.as_id()].insert(GilButton::RightTrigger);
        ret.buttons[Button::C.as_id()].insert(GilButton::East);
        ret.buttons[Button::D.as_id()].insert(GilButton::South);

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
