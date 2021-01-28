use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};

use crate::axis::DirectedAxis;

pub fn parse(input: &str) -> IResult<&str, DirectedAxis> {
    alt((
        value(DirectedAxis::DownBackward, tag("1")),
        value(DirectedAxis::Down, tag("2")),
        value(DirectedAxis::DownForward, tag("3")),
        value(DirectedAxis::Backward, tag("4")),
        value(DirectedAxis::Neutral, tag("5")),
        value(DirectedAxis::Forward, tag("6")),
        value(DirectedAxis::UpBackward, tag("7")),
        value(DirectedAxis::Up, tag("8")),
        value(DirectedAxis::UpForward, tag("9")),
    ))(input)
}
