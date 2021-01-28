use nom::{
    character::complete::char,
    combinator::{map, success, value, verify},
    IResult, Parser,
};

use crate::button::{Button, ButtonSet};

pub fn parse(input: &str) -> IResult<&str, ButtonSet> {
    verify(
        merge_button(ButtonSet::default(), parse_button('a', Button::A))
            .flat_map(|res| merge_button(res, parse_button('b', Button::B)))
            .flat_map(|res| merge_button(res, parse_button('c', Button::C)))
            .flat_map(|res| merge_button(res, parse_button('d', Button::D)))
            .flat_map(|res| merge_button(res, parse_button('e', Button::E))),
        |buttons| buttons.0 != 0,
    )(input)
}

fn parse_button(c: char, val: Button) -> impl FnMut(&str) -> IResult<&str, Button> {
    move |input| value(val, char(c))(input)
}

fn merge_button<'a, F>(
    buttons: ButtonSet,
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, ButtonSet>
where
    F: Parser<&'a str, Button, nom::error::Error<&'a str>>,
{
    let mut f = map(f, move |b| buttons | b).or(success(buttons));
    move |input| f.parse(input)
}
