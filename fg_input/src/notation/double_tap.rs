use super::directed_axis;
use crate::Input;
use nom::{combinator::verify, IResult, Parser};

pub fn parse(input: &str) -> IResult<&str, Input> {
    directed_axis::parse
        .flat_map(|directed| verify(directed_axis::parse, move |new| &directed == new))
        .map(Input::DoubleTap)
        .parse(input)
}
