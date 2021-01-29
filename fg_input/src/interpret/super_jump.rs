use crate::{axis::Axis, input_state::matches_cardinal};

use super::{
    helper::{next, peek, take_while_m_n, value, verify},
    types::{IResult, InputBuffer, ReadInput},
};

pub fn interpret(motion_size: usize) -> impl FnMut(InputBuffer<'_>) -> IResult<'_, Axis> {
    move |buffer: InputBuffer<'_>| interpret_internal(motion_size, buffer)
}

fn interpret_internal(motion_size: usize, buffer: InputBuffer<'_>) -> IResult<'_, Axis> {
    verify(peek(next), |state| matches_cardinal([0, 1], state.axis))
        .flat_map(|state| {
            let axis = state.axis();
            value(
                axis,
                (
                    take_while_m_n(1, motion_size, move |value| {
                        matches_cardinal([0, 1], value.axis)
                    }),
                    take_while_m_n(0, motion_size, move |value| {
                        !matches_cardinal([0, -1], value.axis)
                            && !matches_cardinal([0, 1], value.axis)
                    }),
                    take_while_m_n(1, motion_size, move |value| {
                        matches_cardinal([0, -1], value.axis)
                    }),
                ),
            )
        })
        .read_input(buffer)
}

#[cfg(test)]
mod test {
    use crate::{axis::Axis, InputState};

    use super::interpret_internal;

    #[test]
    fn test_superjump() {
        let mut buffer = [InputState::default(); 5];

        buffer[0].axis = [1, -1];
        buffer[1].axis = [1, -1];
        buffer[2].axis = [1, -1];
        buffer[3].axis = [1, 1];
        buffer[4].axis = [1, 1];

        assert_eq!(interpret_internal(2, &buffer).unwrap().1, Axis::UpRight);
        assert_eq!(interpret_internal(1, &buffer), None);
    }
    #[test]
    fn test_superjump_with_neutral() {
        let mut buffer = [InputState::default(); 5];

        buffer[0].axis = [1, -1];
        buffer[1].axis = [1, -1];
        buffer[2].axis = [0, 0];
        buffer[3].axis = [1, 1];
        buffer[4].axis = [1, 1];

        assert_eq!(interpret_internal(2, &buffer).unwrap().1, Axis::UpRight);
        assert_eq!(interpret_internal(1, &buffer), None);
    }
    #[test]
    fn test_superjump_multi_upper() {
        let mut buffer = [InputState::default(); 5];

        buffer[0].axis = [1, -1];
        buffer[1].axis = [1, -1];
        buffer[2].axis = [0, 0];
        buffer[3].axis = [1, 0];
        buffer[4].axis = [1, 1];

        assert_eq!(interpret_internal(2, &buffer).unwrap().1, Axis::UpRight);
        assert_eq!(interpret_internal(1, &buffer), None);
    }
}
