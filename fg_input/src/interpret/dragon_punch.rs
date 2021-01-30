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
    use crate::{
        axis::{Axis, Direction},
        InputState,
    };

    use super::interpret_internal;

    #[test]
    fn test_dpf() {
        let mut buffer = [InputState {
            axis: Default::default(),
            buttons: Default::default(),
        }; 10];

        buffer[0].axis = Axis::DownRight;
        buffer[1].axis = Axis::DownRight;
        buffer[2].axis = Axis::DownRight;
        buffer[3].axis = Axis::Down;
        buffer[4].axis = Axis::Down;
        buffer[5].axis = Axis::Down;
        buffer[6].axis = Axis::Right;

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

        buffer[0].axis = Axis::DownRight;
        buffer[1].axis = Axis::DownRight;
        buffer[2].axis = Axis::DownRight;
        buffer[3].axis = Axis::Down;
        buffer[4].axis = Axis::Down;
        buffer[5].axis = Axis::DownRight;
        buffer[6].axis = Axis::Right;

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

        buffer[0].axis = Axis::DownRight;
        buffer[1].axis = Axis::DownRight;
        buffer[2].axis = Axis::DownRight;
        buffer[3].axis = Axis::Down;
        buffer[4].axis = Axis::Neutral;
        buffer[5].axis = Axis::DownRight;
        buffer[6].axis = Axis::Right;

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
        buffer[0].axis = Axis::UpRight;
        // 6
        buffer[1].axis = Axis::Right;
        buffer[2].axis = Axis::Right;
        buffer[3].axis = Axis::Right;
        // 3
        buffer[4].axis = Axis::DownRight;
        buffer[5].axis = Axis::DownRight;
        // 2
        buffer[6].axis = Axis::Down;
        buffer[7].axis = Axis::Down;
        buffer[8].axis = Axis::Down;
        // 6
        buffer[9].axis = Axis::Right;

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

        buffer[0].axis = Axis::DownLeft;
        buffer[1].axis = Axis::DownLeft;
        buffer[2].axis = Axis::DownLeft;
        buffer[3].axis = Axis::Down;
        buffer[4].axis = Axis::Down;
        buffer[5].axis = Axis::DownLeft;
        buffer[6].axis = Axis::Left;

        let buffer: Vec<_> = buffer.iter().rev().copied().collect();

        let (_, result) = interpret_internal(8, &buffer).unwrap();
        assert_eq!(result, Direction::Backward);

        assert_eq!(interpret_internal(1, &buffer), None);
    }
}
