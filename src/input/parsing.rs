use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::char,
    combinator::opt,
    combinator::{map, map_parser, success, value, verify},
    sequence,
    sequence::pair,
    IResult, Parser,
};
use sequence::{preceded, tuple};

use super::{
    button::{Button, ButtonSet},
    DirectedAxis, Direction, Input,
};

pub fn parse_input(input: &str) -> IResult<&str, Input> {
    alt((
        parse_high_jump,
        parse_double_tap,
        parse_quarter_circle,
        parse_dragon_punch,
        parse_button_press,
        parse_idle,
    ))(input)
}

pub fn parse_high_jump(input: &str) -> IResult<&str, Input> {
    preceded(
        alt((tag("hj"), tag("2"), tag("1"), tag("3"))),
        map_parser(alt((tag("7"), tag("8"), tag("9"))), parse_directed_axis).map(Input::SuperJump),
    )(input)
}
pub fn parse_quarter_circle(input: &str) -> IResult<&str, Input> {
    map(
        pair(
            alt((
                tag("236").map(|_| Direction::Forward),
                tag("214").map(|_| Direction::Backward),
            )),
            parse_button_set,
        ),
        |(dir, buttons)| Input::QuarterCircle(dir, buttons),
    )(input)
}
pub fn parse_dragon_punch(input: &str) -> IResult<&str, Input> {
    map(
        pair(
            alt((
                tag("623").map(|_| Direction::Forward),
                tag("421").map(|_| Direction::Backward),
            )),
            parse_button_set,
        ),
        |(dir, buttons)| Input::DragonPunch(dir, buttons),
    )(input)
}

pub fn parse_idle(input: &str) -> IResult<&str, Input> {
    map(parse_directed_axis, Input::Idle)(input)
}

pub fn parse_double_tap(input: &str) -> IResult<&str, Input> {
    parse_directed_axis
        .flat_map(|directed| verify(parse_directed_axis, move |new| &directed == new))
        .map(Input::DoubleTap)
        .parse(input)
}

pub fn parse_button_press(input: &str) -> IResult<&str, Input> {
    pair(parse_directed_axis, parse_button_set)
        .map(|(axis, buttons)| Input::PressButton(axis, buttons))
        .parse(input)
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

#[allow(clippy::many_single_char_names)]
pub fn parse_button_set(input: &str) -> IResult<&str, ButtonSet> {
    verify(
        merge_button(ButtonSet::default(), parse_button('a', Button::A))
            .flat_map(|res| merge_button(res, parse_button('b', Button::B)))
            .flat_map(|res| merge_button(res, parse_button('c', Button::C)))
            .flat_map(|res| merge_button(res, parse_button('d', Button::D)))
            .flat_map(|res| merge_button(res, parse_button('e', Button::E))),
        |buttons| buttons.0 != 0,
    )(input)
}

pub fn parse_directed_axis(input: &str) -> IResult<&str, DirectedAxis> {
    let (input, s) = alt((
        tag("1"),
        tag("2"),
        tag("3"),
        tag("4"),
        tag("5"),
        tag("6"),
        tag("7"),
        tag("8"),
        tag("9"),
    ))(input)?;
    Ok((
        input,
        match s {
            "8" => DirectedAxis::Up,
            "2" => DirectedAxis::Down,
            "6" => DirectedAxis::Forward,
            "4" => DirectedAxis::Backward,
            "5" => DirectedAxis::Neutral,
            "9" => DirectedAxis::UpForward,
            "7" => DirectedAxis::UpBackward,
            "3" => DirectedAxis::DownForward,
            "1" => DirectedAxis::DownBackward,
            _ => unreachable!(),
        },
    ))
}
