use crate::axis::Axis;

use super::{
    axis::axes,
    helper::{alt, take_while_m_n, value},
    types::{IResult, InputBuffer},
};

pub fn interpret(motion_size: usize) -> impl FnMut(InputBuffer<'_>) -> IResult<'_, Axis> {
    move |buffer: InputBuffer<'_>| interpret_internal(motion_size, buffer)
}
fn interpret_internal(motion_size: usize, buffer: InputBuffer<'_>) -> IResult<'_, Axis> {
    let (required, _) = axes(motion_size);

    alt((
        value(
            Axis::Right,
            (
                required(Axis::Right),
                take_while_m_n(1, motion_size, move |value| {
                    !(value.axis & Axis::Right == Axis::Right)
                }),
                required(Axis::Right),
            ),
        ),
        value(
            Axis::Left,
            (
                required(Axis::Left),
                take_while_m_n(1, motion_size, move |value| {
                    !(value.axis & Axis::Left == Axis::Left)
                }),
                required(Axis::Left),
            ),
        ),
        value(
            Axis::Down,
            (
                required(Axis::Down),
                take_while_m_n(1, motion_size, move |value| {
                    !(value.axis & Axis::Down == Axis::Down)
                }),
                required(Axis::Down),
            ),
        ),
    ))(buffer)
}

#[cfg(test)]
mod test {
    use crate::{axis::Axis, InputState};

    use super::interpret_internal;

    #[test]
    fn test_double_tap() {
        let mut buffer = [InputState::default(); 6];

        buffer[0].axis = Axis::Right;
        buffer[1].axis = Axis::Right;
        buffer[2].axis = Axis::Right;
        buffer[3].axis = Axis::Neutral;
        buffer[4].axis = Axis::Right;
        buffer[5].axis = Axis::Right;

        assert_eq!(interpret_internal(2, &buffer).unwrap().1, Axis::Right);
        assert_eq!(interpret_internal(1, &buffer), None);

        buffer[2].axis = Axis::DownRight;
        buffer[3].axis = Axis::UpRight;

        assert_eq!(interpret_internal(2, &buffer), None);
    }
}
