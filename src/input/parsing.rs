use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::opt,
    combinator::{peek, success, verify},
    IResult,
};

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
    let (input, _) = alt((tag("hj"), tag("2"), tag("1"), tag("3")))(input)?;
    let (input, _) = peek(alt((tag("7"), tag("8"), tag("9"))))(input)?;
    let (input, dir) = parse_directed_axis(input)?;
    Ok((input, Input::SuperJump(dir)))
}
pub fn parse_quarter_circle(input: &str) -> IResult<&str, Input> {
    let (input, dir) = alt((tag("236"), tag("214")))(input)?;

    let dir = match dir {
        "236" => Direction::Forward,
        "214" => Direction::Backward,
        _ => unreachable!(),
    };

    let (input, buttons) = parse_button_set(input)?;

    Ok((input, Input::QuarterCircle(dir, buttons)))
}
pub fn parse_dragon_punch(input: &str) -> IResult<&str, Input> {
    let (input, dir) = alt((tag("623"), tag("421")))(input)?;

    let dir = match dir {
        "623" => Direction::Forward,
        "421" => Direction::Backward,
        _ => unreachable!(),
    };

    let (input, buttons) = parse_button_set(input)?;

    Ok((input, Input::DragonPunch(dir, buttons)))
}

pub fn parse_idle(input: &str) -> IResult<&str, Input> {
    let (input, axis) = parse_directed_axis(input)?;
    Ok((input, Input::Idle(axis)))
}

pub fn parse_double_tap(input: &str) -> IResult<&str, Input> {
    let (input, axis) = parse_directed_axis(input)?;
    let (input, _) = verify(parse_directed_axis, |second| &axis == second)(input)?;
    Ok((input, Input::DoubleTap(axis)))
}

pub fn parse_button_press(input: &str) -> IResult<&str, Input> {
    let (input, axis) = parse_directed_axis(input)?;
    let (input, buttons) = parse_button_set(input)?;
    Ok((input, Input::PressButton(axis, buttons)))
}

#[allow(clippy::many_single_char_names)]
pub fn parse_button_set(input: &str) -> IResult<&str, ButtonSet> {
    let mut buttons = ButtonSet::default();

    let (input, a) = opt(tag("a"))(input)?;
    if a.is_some() {
        buttons |= Button::A;
    }

    let (input, b) = opt(tag("b"))(input)?;
    if b.is_some() {
        buttons |= Button::B;
    }

    let (input, c) = opt(tag("c"))(input)?;
    if c.is_some() {
        buttons |= Button::C;
    }

    let (input, d) = opt(tag("d"))(input)?;
    if d.is_some() {
        buttons |= Button::D;
    }

    let (input, e) = opt(tag("e"))(input)?;
    if e.is_some() {
        buttons |= Button::E;
    }

    let (input, _) = verify(success(0), |value| &buttons.0 != value)(input)?;

    Ok((input, buttons))
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
