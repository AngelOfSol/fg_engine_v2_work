use super::Button;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use std::{
    ops::{BitOr, BitOrAssign},
    str::FromStr,
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, Inspect, Default)]
pub struct ButtonSet(u8);

impl From<Button> for ButtonSet {
    fn from(value: Button) -> ButtonSet {
        ButtonSet(value as u8)
    }
}
pub mod parse {
    use pest_derive::Parser;
    #[derive(Parser)]
    #[grammar_inline = r#"
        a = { "a" }
        b = { "b" }
        c = { "c" }
        d = { "d" }
        e = { "e" }
        valid = { a? ~ b? ~ c? ~ d? ~ e? ~ EOI}
    "#]
    pub struct ButtonSetParser;
}

use parse::*;
use pest::Parser;

impl FromStr for ButtonSet {
    type Err = pest::error::Error<parse::Rule>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = parse::ButtonSetParser::parse(Rule::valid, s)?
            .next()
            .unwrap()
            .into_inner();
        let mut buttons = Self(0);

        for pair in pairs {
            match pair.as_rule() {
                Rule::a => buttons |= Button::A,
                Rule::b => buttons |= Button::B,
                Rule::c => buttons |= Button::C,
                Rule::d => buttons |= Button::D,
                Rule::e => buttons |= Button::E,
                Rule::valid => unreachable!(),
                Rule::EOI => (),
            }
        }

        Ok(buttons)
    }
}

#[cfg(test)]
mod test {
    use super::ButtonSet;
    use crate::input::button::Button;
    use std::str::FromStr;

    #[test]
    fn single_button() {
        assert_eq!(ButtonSet::from_str("a"), Ok(ButtonSet::from(Button::A)));
        assert_eq!(ButtonSet::from_str("b"), Ok(ButtonSet::from(Button::B)));
        assert_eq!(ButtonSet::from_str("c"), Ok(ButtonSet::from(Button::C)));
        assert_eq!(ButtonSet::from_str("d"), Ok(ButtonSet::from(Button::D)));
        assert_eq!(ButtonSet::from_str("e"), Ok(ButtonSet::from(Button::E)));
    }
    #[test]
    fn multi_button() {
        assert_eq!(
            ButtonSet::from_str("ab"),
            Ok(ButtonSet::from(Button::A) | Button::B)
        );
        assert_eq!(
            ButtonSet::from_str("abcde"),
            Ok(ButtonSet::from(Button::A) | Button::B | Button::C | Button::D | Button::E)
        );
        assert_eq!(
            ButtonSet::from_str("ace"),
            Ok(ButtonSet::from(Button::A) | Button::C | Button::E)
        );
        assert!(ButtonSet::from_str("acb").is_err());
        assert!(ButtonSet::from_str("aa").is_err());
    }
}

impl<Rhs> BitOr<Rhs> for ButtonSet
where
    Rhs: Into<ButtonSet>,
{
    type Output = ButtonSet;
    fn bitor(self, rhs: Rhs) -> Self::Output {
        ButtonSet(self.0 | rhs.into().0)
    }
}

impl<Rhs> BitOrAssign<Rhs> for ButtonSet
where
    Rhs: Into<ButtonSet>,
{
    fn bitor_assign(&mut self, rhs: Rhs) {
        self.0 |= rhs.into().0
    }
}
