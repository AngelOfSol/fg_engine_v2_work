use crate::button::ButtonSet;

use super::{
    helper::{map, next, take},
    types::{IResult, InputBuffer},
};

pub fn interpret_buffered(
    grace_period: usize,
) -> impl FnMut(InputBuffer<'_>) -> IResult<'_, ButtonSet> {
    assert!(grace_period != 0);
    move |buffer: InputBuffer| {
        let (mut buffer, mut button_set) = interpret(buffer)?;
        for _ in 1..grace_period {
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
    map(next, |state| state.just_pressed())(buffer)
}

#[cfg(test)]
mod test {
    use button_set::ButtonSet;

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
            Some((&buffer[..4], button_set::A | button_set::B))
        );
        assert_eq!(
            interpret(&buffer[..1]),
            Some((&buffer[..0], ButtonSet::default()))
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
            Some((&buffer[..2], button_set::A | button_set::B))
        );
    }
}
