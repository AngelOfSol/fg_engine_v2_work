use super::{button_set, directed_axis};
use crate::Input;
use nom::{sequence::pair, IResult, Parser};

pub fn parse(input: &str) -> IResult<&str, Input> {
    pair(directed_axis::parse, button_set::parse)
        .map(|(axis, buttons)| Input::PressButton(buttons, axis))
        .parse(input)
}
