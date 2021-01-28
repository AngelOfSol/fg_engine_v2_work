use crate::Input;
use nom::{branch::alt, IResult};

pub mod button_press;
pub mod button_set;
pub mod directed_axis;
pub mod double_tap;
pub mod dragon_punch;
pub mod high_jump;
pub mod idle;
pub mod quarter_circle;

pub fn parse(input: &str) -> IResult<&str, Input> {
    alt((
        quarter_circle::parse,
        dragon_punch::parse,
        high_jump::parse,
        double_tap::parse,
        button_press::parse,
        idle::parse,
    ))(input)
}
