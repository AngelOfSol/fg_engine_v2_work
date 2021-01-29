mod alt;

use crate::{axis::Axis, InputState};
pub use alt::alt;

use super::types::{IResult, InputBuffer, ReadInput};

pub fn next(buffer: InputBuffer) -> IResult<'_, &InputState> {
    let element = buffer.last()?;
    Some((&buffer[..buffer.len() - 1], element))
}

pub fn take(count: usize) -> impl FnMut(InputBuffer<'_>) -> IResult<'_> {
    move |buffer: InputBuffer| {
        if count > buffer.len() {
            None
        } else {
            Some(buffer.split_at(buffer.len() - count))
        }
    }
}

pub fn take_while_m_n<'a, F>(
    m: usize,
    n: usize,
    mut pred: F,
) -> impl FnMut(InputBuffer<'a>) -> IResult<'a>
where
    F: FnMut(&InputState) -> bool,
{
    assert!(m <= n);

    move |buffer: InputBuffer| {
        let position = buffer.iter().rev().position(|value| !pred(value));
        match position {
            Some(idx) => {
                if idx >= m {
                    Some(buffer.split_at(buffer.len() - n.min(idx)))
                } else {
                    None
                }
            }
            None => {
                if buffer.len() >= m {
                    Some(buffer.split_at(buffer.len() - n.min(buffer.len())))
                } else {
                    None
                }
            }
        }
    }
}

pub fn map<'a, R, F, O1, O2>(mut first: R, map: F) -> impl FnMut(InputBuffer<'a>) -> IResult<'a, O2>
where
    R: ReadInput<'a, O1>,
    F: Fn(O1) -> O2,
{
    move |buffer: InputBuffer| {
        let (buffer, result) = first.read_input(buffer)?;
        Some((buffer, map(result)))
    }
}

pub fn success<'a, O>(value: O) -> impl FnMut(InputBuffer<'a>) -> IResult<'a, O>
where
    O: Clone,
{
    move |buffer: InputBuffer| Some((buffer, value.clone()))
}

pub fn peek<'a, F, O>(mut f: F) -> impl FnMut(InputBuffer<'a>) -> IResult<'a, O>
where
    F: ReadInput<'a, O>,
{
    move |buffer: InputBuffer| {
        let (_, result) = f.read_input(buffer)?;
        Some((buffer, result))
    }
}

pub fn value<'a, R, O, O2>(value: O, mut r: R) -> impl FnMut(InputBuffer<'a>) -> IResult<'a, O>
where
    O: Clone,
    R: ReadInput<'a, O2>,
{
    move |buffer: InputBuffer| {
        let (buffer, _) = r.read_input(buffer)?;
        Some((buffer, value.clone()))
    }
}

pub fn next_axis(buffer: InputBuffer<'_>) -> IResult<'_, Axis> {
    map(next, |state| state.axis())(buffer)
}

pub fn verify<'a, R, O>(
    mut r: R,
    cond: impl Fn(&O) -> bool,
) -> impl FnMut(InputBuffer<'a>) -> IResult<'a, O>
where
    R: ReadInput<'a, O>,
{
    move |buffer: InputBuffer| {
        let (buffer, value) = r.read_input(buffer)?;
        if cond(&value) {
            Some((buffer, value))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{axis::Axis, InputState};

    use super::{map, next, next_axis, peek, success, take, take_while_m_n, value, verify};

    #[test]
    fn test_success() {
        let buffer = [Default::default(); 1];

        assert_eq!(success(0)(&buffer), Some((buffer.as_ref(), 0)));
    }

    #[test]
    fn test_next() {
        let buffer = [Default::default(); 1];

        let (buffer, output) = next(&buffer).expect("expect to have one item after next");

        assert_eq!(output, &Default::default());
        assert!(buffer.is_empty());

        assert_eq!(next(buffer), None);
    }
    #[test]
    fn test_take() {
        let buffer = [Default::default(); 2];

        let (new_buffer, result) = take(1)(&buffer).unwrap();
        assert_eq!(&buffer[0..1], new_buffer);
        assert_eq!(&buffer[1..2], result);

        assert_eq!(take(3)(&buffer), None);
    }

    #[test]
    fn test_take_while_m_n() {
        let mut buffer = [InputState::default(); 10];
        buffer[buffer.len() - 1].axis = [1, 1];
        let buffer = buffer;

        assert_eq!(
            take_while_m_n(0, 3, |state| state.axis() == Axis::UpRight)(&buffer),
            Some((&buffer[0..buffer.len() - 1], &buffer[buffer.len() - 1..]))
        );

        assert_eq!(
            take_while_m_n(2, 3, |state| state.axis() == Axis::UpRight)(&buffer),
            None
        );

        assert_eq!(
            take_while_m_n(3, 5, |_| true)(&buffer),
            Some((&buffer[0..5], &buffer[5..]))
        );

        assert_eq!(
            take_while_m_n(10, 20, |_| true)(&buffer),
            Some((&buffer[..0], &buffer[..]))
        );
    }

    #[test]
    fn test_map() {
        let buffer = [Default::default()];

        assert_eq!(
            map(success(1), |val| val + 1)(&buffer),
            Some((buffer.as_ref(), 2))
        );
    }

    #[test]
    fn test_peek() {
        let buffer = [Default::default()];

        assert_eq!(peek(next)(&buffer), Some((buffer.as_ref(), &buffer[0])));
        assert_ne!(next(&buffer), peek(next)(&buffer));
    }

    #[test]
    fn test_value() {
        let buffer = [Default::default()];

        assert_eq!(value(0, take(1))(&buffer), Some((&buffer[..0], 0)));
        assert_eq!(value(0, take(2))(&buffer), None);
    }

    #[test]
    fn test_next_axis() {
        let buffer = [Default::default(); 1];

        let (buffer, output) = next_axis(&buffer).expect("expect to have one item after next");

        assert_eq!(output, Axis::Neutral);
        assert!(buffer.is_empty());

        assert_eq!(next_axis(buffer), None);
    }

    #[test]
    fn test_verify() {
        let buffer = [Default::default(); 1];

        assert_eq!(
            verify(next_axis, |axis| *axis == Axis::Neutral)(&buffer),
            Some((&buffer[..0], Axis::Neutral))
        );
        assert_eq!(verify(next_axis, |axis| *axis == Axis::Up)(&buffer), None);
    }
}
