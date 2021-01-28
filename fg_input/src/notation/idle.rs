use super::directed_axis;
use crate::Input;
use nom::{combinator::map, IResult};

pub fn parse(input: &str) -> IResult<&str, Input> {
    map(directed_axis::parse, Input::Idle)(input)
}
