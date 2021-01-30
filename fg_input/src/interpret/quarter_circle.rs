use crate::axis::{Axis, Direction};

use super::{
    axis::axes,
    helper::{alt, value},
    types::{IResult, InputBuffer},
};

pub fn interpret(motion_size: usize) -> impl FnMut(InputBuffer<'_>) -> IResult<'_, Direction> {
    move |buffer: InputBuffer<'_>| interpret_internal(motion_size, buffer)
}

fn interpret_internal(motion_size: usize, buffer: InputBuffer<'_>) -> IResult<'_, Direction> {
    assert!(motion_size > 0);

    let (required, optional) = axes(motion_size);

    alt((
        value(
            Direction::Forward,
            (
                optional(Axis::UpRight),
                required(Axis::Right),
                required(Axis::DownRight),
                required(Axis::Down),
            ),
        ),
        (value(
            Direction::Backward,
            (
                optional(Axis::UpLeft),
                required(Axis::Left),
                required(Axis::DownLeft),
                required(Axis::Down),
            ),
        )),
    ))(buffer)
}

#[cfg(test)]
mod test {
    use crate::{axis::Direction, InputState};

    use super::interpret_internal;

    #[test]
    fn test_qcf() {
        let mut buffer = [InputState {
            axis: Default::default(),
            buttons: Default::default(),
        }; 10];

        buffer[0].axis = [1, 0];
        buffer[1].axis = [1, 0];
        buffer[2].axis = [1, 0];
        buffer[3].axis = [1, -1];
        buffer[4].axis = [1, -1];
        buffer[5].axis = [1, -1];
        buffer[6].axis = [0, -1];

        let buffer: Vec<_> = buffer.iter().rev().copied().collect();

        let (_, result) = interpret_internal(8, &buffer).unwrap();
        assert_eq!(result, Direction::Forward);

        assert_eq!(interpret_internal(1, &buffer), None);
    }

    #[test]
    fn test_qcf_interrupted() {
        let mut buffer = [InputState {
            axis: Default::default(),
            buttons: Default::default(),
        }; 10];

        buffer[0].axis = [1, 0];
        buffer[1].axis = [1, 0];
        buffer[2].axis = [1, 0];
        buffer[3].axis = [1, -1];
        buffer[4].axis = [-1, 0];
        buffer[5].axis = [1, -1];
        buffer[6].axis = [0, -1];

        let buffer: Vec<_> = buffer.iter().rev().copied().collect();

        assert_eq!(interpret_internal(8, &buffer), None);
    }

    #[test]
    fn test_tk_qcf() {
        let mut buffer = [InputState {
            axis: Default::default(),
            buttons: Default::default(),
        }; 10];

        buffer[0].axis = [1, 1];
        buffer[1].axis = [1, 0];
        buffer[2].axis = [1, 0];
        buffer[3].axis = [1, -1];
        buffer[4].axis = [1, -1];
        buffer[5].axis = [1, -1];
        buffer[6].axis = [0, -1];

        let buffer: Vec<_> = buffer.iter().rev().copied().collect();

        let (_, result) = interpret_internal(8, &buffer).unwrap();
        assert_eq!(result, Direction::Forward);
    }

    #[test]
    fn test_qcb() {
        let mut buffer = [InputState {
            axis: Default::default(),
            buttons: Default::default(),
        }; 10];

        buffer[0].axis = [-1, 0];
        buffer[1].axis = [-1, 0];
        buffer[2].axis = [-1, 0];
        buffer[3].axis = [-1, -1];
        buffer[4].axis = [-1, -1];
        buffer[5].axis = [-1, -1];
        buffer[6].axis = [0, -1];

        let buffer: Vec<_> = buffer.iter().rev().copied().collect();

        let (_, result) = interpret_internal(8, &buffer).unwrap();
        assert_eq!(result, Direction::Backward);
    }
}
