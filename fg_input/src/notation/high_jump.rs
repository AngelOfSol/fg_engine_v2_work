use super::directed_axis;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map_parser, recognize},
    sequence::preceded,
    IResult, Parser,
};

use crate::Input;

pub fn parse(input: &str) -> IResult<&str, Input> {
    preceded(
        alt((recognize(tag("hj")), recognize(one_of("123")))),
        map_parser(alt((tag("7"), tag("8"), tag("9"))), directed_axis::parse).map(Input::SuperJump),
    )(input)
}
