use super::button_set;
use crate::{axis::Direction, Input};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    sequence::pair,
    IResult,
};

pub fn parse(input: &str) -> IResult<&str, Input> {
    map(
        pair(
            alt((
                value(Direction::Forward, tag("236")),
                value(Direction::Backward, tag("214")),
            )),
            button_set::parse,
        ),
        |(dir, buttons)| Input::QuarterCircle(dir, buttons),
    )(input)
}
