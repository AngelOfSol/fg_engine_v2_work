use crate::button::ButtonSet;

use super::{
    helper::{map, next, take},
    types::{IResult, InputBuffer},
};

pub fn interpret_buffered(
    buffer_size: usize,
) -> impl FnMut(InputBuffer<'_>) -> IResult<'_, ButtonSet> {
    move |buffer: InputBuffer| {
        if buffer_size == 0 {
            return None;
        }
        let (mut buffer, mut button_set) = (buffer, ButtonSet::default());
        for _ in 0..buffer_size {
            if let Some((new_buffer, rest)) = take(1)(buffer) {
                buffer = new_buffer;
                if let Some((_, additional)) = interpret(rest) {
                    button_set |= additional;
                }
            } else {
                break;
            }
        }
        Some((buffer, button_set))
    }
}

pub fn interpret(buffer: InputBuffer<'_>) -> IResult<'_, ButtonSet> {
    let (buffer, buttons) = map(next, |state| state.just_pressed())(buffer)?;
    if buttons.is_empty() {
        None
    } else {
        Some((buffer, buttons))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        button::{button_set, ButtonState},
        InputState,
    };

    use super::{interpret, interpret_buffered};

    #[test]
    fn test_button_set() {
        let mut buffer = [InputState {
            axis: Default::default(),
            buttons: [ButtonState::Released; 5],
        }; 5];

        buffer[4].buttons[0] = ButtonState::JustPressed;
        buffer[4].buttons[1] = ButtonState::JustPressed;
        buffer[3].buttons[2] = ButtonState::JustPressed;

        let buffer = buffer;

        assert_eq!(
            interpret(&buffer),
            Some((&buffer[0..4], button_set::A | button_set::B))
        );
    }
    #[test]
    fn test_button_set_buffered() {
        let mut buffer = [InputState {
            axis: Default::default(),
            buttons: [ButtonState::Released; 5],
        }; 5];

        buffer[4].buttons[0] = ButtonState::JustPressed;
        buffer[2].buttons[1] = ButtonState::JustPressed;
        buffer[0].buttons[2] = ButtonState::JustPressed;

        let buffer = buffer;

        assert_eq!(
            interpret_buffered(3)(&buffer),
            Some((&buffer[0..2], button_set::A | button_set::B))
        );
    }
}
