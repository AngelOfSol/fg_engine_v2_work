use nom::{
    character::complete::char,
    combinator::{map, success, value, verify},
    IResult, Parser,
};

use crate::button::{button_set, ButtonSet};

pub fn parse(input: &str) -> IResult<&str, ButtonSet> {
    verify(
        merge_button(ButtonSet::default(), parse_button('a', button_set::A))
            .flat_map(|res| merge_button(res, parse_button('b', button_set::B)))
            .flat_map(|res| merge_button(res, parse_button('c', button_set::C)))
            .flat_map(|res| merge_button(res, parse_button('d', button_set::D)))
            .flat_map(|res| merge_button(res, parse_button('e', button_set::E))),
        |buttons| buttons.0 != 0,
    )(input)
}

fn parse_button(c: char, val: ButtonSet) -> impl FnMut(&str) -> IResult<&str, ButtonSet> {
    move |input| value(val, char(c))(input)
}

fn merge_button<'a, F>(
    buttons: ButtonSet,
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, ButtonSet>
where
    F: Parser<&'a str, ButtonSet, nom::error::Error<&'a str>>,
{
    let mut f = map(f, move |b| buttons | b).or(success(buttons));
    move |input| f.parse(input)
}
