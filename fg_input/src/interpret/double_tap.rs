use crate::{axis::Axis, input_state::matches_cardinal};

use super::{
    axis::axes,
    helper::{next, peek, take_while_m_n, value},
    types::{IResult, InputBuffer, ReadInput},
};

pub fn interpret(motion_size: usize, buffer: InputBuffer<'_>) -> IResult<'_, Axis> {
    let (required, _) = axes(motion_size);

    peek(next)
        .flat_map(|state| {
            let axis = state.axis();
            value(
                axis,
                (
                    required(axis),
                    take_while_m_n(1, motion_size, move |value| {
                        !matches_cardinal(state.axis, value.axis)
                    }),
                    required(axis),
                ),
            )
        })
        .read_input(buffer)
}

#[cfg(test)]
mod test {
    use crate::{axis::Axis, InputState};

    use super::{interpret, matches_cardinal};

    #[test]
    fn test_matches_cardinal() {
        let up = [0, 1];
        let upright = [1, 1];
        let neutral = [0, 0];
        let right = [1, 0];

        assert!(!matches_cardinal(up, neutral));
        assert!(matches_cardinal(up, upright));
        assert!(matches_cardinal(up, up));
        assert!(!matches_cardinal(right, neutral));
        assert!(matches_cardinal(right, upright));
    }

    #[test]
    fn test_double_tap() {
        let mut buffer = [InputState::default(); 6];

        buffer[0].axis = [1, 0];
        buffer[1].axis = [1, 0];
        buffer[2].axis = [1, 0];
        buffer[3].axis = [0, 0];
        buffer[4].axis = [1, 0];
        buffer[5].axis = [1, 0];

        assert_eq!(interpret(2, &buffer).unwrap().1, Axis::Right);
        assert_eq!(interpret(1, &buffer), None);

        buffer[2].axis = [1, -1];
        buffer[3].axis = [1, 1];

        assert_eq!(interpret(2, &buffer), None);
    }
}
