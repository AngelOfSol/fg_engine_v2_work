use crate::axis::Axis;

use super::{
    axis::axes,
    helper::{alt, map, take_while_m_n},
    types::{IResult, InputBuffer},
};

pub fn interpret(motion_size: usize) -> impl FnMut(InputBuffer<'_>) -> IResult<'_, Axis> {
    move |buffer: InputBuffer<'_>| interpret_internal(motion_size, buffer)
}

// !(value.axis & Axis::Up || value.axis & Axis::Down)
fn interpret_internal(motion_size: usize, buffer: InputBuffer<'_>) -> IResult<'_, Axis> {
    let (required, _) = axes(motion_size);

    map(
        (
            take_while_m_n(1, motion_size, move |value| {
                value.axis & Axis::Up != Axis::Neutral
            }),
            take_while_m_n(0, motion_size, move |value| {
                value.axis & Axis::Up == Axis::Neutral && value.axis & Axis::Down == Axis::Neutral
            }),
            alt((
                required(Axis::DownRight),
                required(Axis::Down),
                required(Axis::DownLeft),
            )),
        ),
        |(axis, _, _)| axis.last().unwrap().axis,
    )(buffer)
}

#[cfg(test)]
mod test {
    use crate::{axis::Axis, InputState};

    use super::interpret_internal;

    #[test]
    fn test_superjump() {
        let mut buffer = [InputState::default(); 5];

        buffer[0].axis = Axis::DownRight;
        buffer[1].axis = Axis::DownRight;
        buffer[2].axis = Axis::DownRight;
        buffer[3].axis = Axis::UpRight;
        buffer[4].axis = Axis::UpRight;

        assert_eq!(interpret_internal(2, &buffer).unwrap().1, Axis::UpRight);
        assert_eq!(interpret_internal(1, &buffer), None);
    }
    #[test]
    fn test_superjump_with_neutral() {
        let mut buffer = [InputState::default(); 5];

        buffer[0].axis = Axis::DownRight;
        buffer[1].axis = Axis::DownRight;
        buffer[2].axis = Axis::Neutral;
        buffer[3].axis = Axis::UpRight;
        buffer[4].axis = Axis::UpRight;

        assert_eq!(interpret_internal(2, &buffer).unwrap().1, Axis::UpRight);
        assert_eq!(interpret_internal(1, &buffer), None);
    }
    #[test]
    fn test_superjump_multi_upper() {
        let mut buffer = [InputState::default(); 6];

        buffer[0].axis = Axis::DownRight;
        buffer[1].axis = Axis::DownRight;
        buffer[2].axis = Axis::Neutral;
        buffer[3].axis = Axis::Left;
        buffer[4].axis = Axis::Right;
        buffer[5].axis = Axis::UpRight;

        assert_eq!(interpret_internal(3, &buffer).unwrap().1, Axis::UpRight);
        assert_eq!(interpret_internal(1, &buffer), None);
    }
}
