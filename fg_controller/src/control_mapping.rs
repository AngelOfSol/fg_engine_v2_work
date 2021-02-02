use crate::backend::{Button, ControllerState};
use fg_input::{
    axis::Axis,
    button::{button_set, ButtonSet},
    InputState,
};
use maplit::hashmap;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
pub struct ControlMapping {
    pub buttons: HashMap<Button, ButtonSet>,
}

impl Default for ControlMapping {
    fn default() -> Self {
        Self {
            buttons: hashmap! {
                Button::X => button_set::A,
                Button::Y => button_set::B,
                Button::B => button_set::C,
                Button::A => button_set::D,
                Button::R1 => button_set::E,
                Button::R2 => button_set::A | button_set::B,
                Button::L2 => button_set::C | button_set::D,
            },
        }
    }
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

    pub fn find_button(&self, button_set: &ButtonSet) -> Option<Button> {
        self.buttons
            .iter()
            .find(|(_, set)| *set == button_set)
            .map(|(button, _)| *button)
    }
}

#[cfg(test)]
mod test {
    use fg_input::{axis::Axis, button::ButtonState, InputState};

    use crate::backend::{Button, ControllerState};

    use super::ControlMapping;

    #[test]
    fn test_map() {
        let mapping = ControlMapping::default();

        let first = InputState::default();
        let controls = ControllerState {
            buttons: Default::default(),
            dpad: Default::default(),
            left_stick: Default::default(),
            right_stick: Default::default(),
        };

        assert_eq!(mapping.map(first, &controls), first);

        let controls = ControllerState {
            dpad: Axis::Up,
            ..controls
        };

        assert_eq!(
            mapping.map(first, &controls),
            InputState {
                axis: Axis::Up,
                ..first
            }
        );

        let mut controls = ControllerState {
            dpad: Axis::Up,
            ..controls
        };

        controls[Button::A] = true;

        let mut second = InputState {
            axis: Axis::Up,
            ..first
        };

        second.buttons[3] = ButtonState::JustPressed;

        let mut third = InputState {
            axis: Axis::Up,
            ..second
        };

        third.buttons[3] = ButtonState::Pressed;

        assert_eq!(mapping.map(first, &controls), second);
        assert_eq!(mapping.map(second, &controls), third);

        let controls = ControllerState {
            ..Default::default()
        };

        let mut second_test = InputState {
            axis: Axis::Neutral,
            ..first
        };

        second_test.buttons[3] = ButtonState::JustReleased;

        assert_eq!(mapping.map(second, &controls), second_test);
        assert_eq!(mapping.map(third, &controls), second_test);

        let controls = ControllerState {
            dpad: Axis::UpRight,
            ..Default::default()
        };
        assert_eq!(
            mapping.map(first, &controls),
            InputState {
                axis: Axis::UpRight,
                ..first
            }
        );
    }
}
