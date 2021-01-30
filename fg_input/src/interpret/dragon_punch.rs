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
                optional(Axis::Right),
                required(Axis::DownRight),
                required(Axis::Down),
                optional(Axis::DownRight),
                required(Axis::Right),
            ),
        ),
        (value(
            Direction::Backward,
            (
                optional(Axis::UpLeft),
                optional(Axis::Left),
                required(Axis::DownLeft),
                required(Axis::Down),
                optional(Axis::DownLeft),
                required(Axis::Left),
            ),
        )),
    ))(buffer)
}

#[cfg(test)]
mod test {
    use crate::{axis::Direction, InputState};

    use super::interpret_internal;

    #[test]
    fn test_dpf() {
        let mut buffer = [InputState {
            axis: Default::default(),
            buttons: Default::default(),
        }; 10];

        buffer[0].axis = [1, -1];
        buffer[1].axis = [1, -1];
        buffer[2].axis = [1, -1];
        buffer[3].axis = [0, -1];
        buffer[4].axis = [0, -1];
        buffer[5].axis = [0, -1];
        buffer[6].axis = [1, 0];

        let buffer: Vec<_> = buffer.iter().rev().copied().collect();

        let (_, result) = interpret_internal(8, &buffer).unwrap();
        assert_eq!(result, Direction::Forward);

        assert_eq!(interpret_internal(1, &buffer), None);
    }
    #[test]
    fn test_dpf_extended() {
        let mut buffer = [InputState {
            axis: Default::default(),
            buttons: Default::default(),
        }; 10];

        buffer[0].axis = [1, -1];
        buffer[1].axis = [1, -1];
        buffer[2].axis = [1, -1];
        buffer[3].axis = [0, -1];
        buffer[4].axis = [0, -1];
        buffer[5].axis = [1, -1];
        buffer[6].axis = [1, 0];

        let buffer: Vec<_> = buffer.iter().rev().copied().collect();

        let (_, result) = interpret_internal(8, &buffer).unwrap();
        assert_eq!(result, Direction::Forward);

        assert_eq!(interpret_internal(1, &buffer), None);
    }

    #[test]
    fn test_dpf_interrupted() {
        let mut buffer = [InputState {
            axis: Default::default(),
            buttons: Default::default(),
        }; 10];

        buffer[0].axis = [1, -1];
        buffer[1].axis = [1, -1];
        buffer[2].axis = [1, -1];
        buffer[3].axis = [0, -1];
        buffer[4].axis = [0, 0];
        buffer[5].axis = [1, -1];
        buffer[6].axis = [1, 0];

        let buffer: Vec<_> = buffer.iter().rev().copied().collect();

        assert_eq!(interpret_internal(8, &buffer), None);
    }

    #[test]
    fn test_tk_dpf() {
        let mut buffer = [InputState {
            axis: Default::default(),
            buttons: Default::default(),
        }; 10];

        // 9
        buffer[0].axis = [1, 1];
        // 6
        buffer[1].axis = [1, 0];
        buffer[2].axis = [1, 0];
        buffer[3].axis = [1, 0];
        // 3
        buffer[4].axis = [1, -1];
        buffer[5].axis = [1, -1];
        // 2
        buffer[6].axis = [0, -1];
        buffer[7].axis = [0, -1];
        buffer[8].axis = [0, -1];
        // 6
        buffer[9].axis = [1, 0];

        let buffer: Vec<_> = buffer.iter().rev().copied().collect();

        let (_, result) = interpret_internal(8, &buffer).unwrap();
        assert_eq!(result, Direction::Forward);
    }

    #[test]
    fn test_dpb() {
        let mut buffer = [InputState {
            axis: Default::default(),
            buttons: Default::default(),
        }; 10];

        buffer[0].axis = [-1, -1];
        buffer[1].axis = [-1, -1];
        buffer[2].axis = [-1, -1];
        buffer[3].axis = [0, -1];
        buffer[4].axis = [0, -1];
        buffer[5].axis = [-1, -1];
        buffer[6].axis = [-1, 0];

        let buffer: Vec<_> = buffer.iter().rev().copied().collect();

        let (_, result) = interpret_internal(8, &buffer).unwrap();
        assert_eq!(result, Direction::Backward);

        assert_eq!(interpret_internal(1, &buffer), None);
    }
}
