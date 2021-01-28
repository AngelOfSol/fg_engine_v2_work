use super::button_set;
use crate::{Direction, Input};
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
                value(Direction::Forward, tag("623")),
                value(Direction::Backward, tag("421")),
            )),
            button_set::parse,
        ),
        |(dir, buttons)| Input::DragonPunch(dir, buttons),
    )(input)
}
