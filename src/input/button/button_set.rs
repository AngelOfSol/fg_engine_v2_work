use crate::input::parsing::parse_button_set;

use super::Button;
use inspect_design::Inspect;
use nom::Finish;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    ops::{BitOr, BitOrAssign},
    str::FromStr,
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, Inspect, Default)]
pub struct ButtonSet(pub u8);

impl ButtonSet {
    pub fn has(&self, button: Button) -> bool {
        self.0 & button as u8 == button as u8
    }
}

impl From<Button> for ButtonSet {
    fn from(value: Button) -> ButtonSet {
        ButtonSet(value as u8)
    }
}

impl FromStr for ButtonSet {
    type Err = nom::error::ErrorKind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_button_set(s)
            .finish()
            .map_err(|err| err.code)
            .map(|(_, item)| item)
    }
}

impl Display for ButtonSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.has(Button::A) {
            write!(f, "a")?
        }
        if self.has(Button::B) {
            write!(f, "b")?
        }
        if self.has(Button::C) {
            write!(f, "c")?
        }
        if self.has(Button::D) {
            write!(f, "d")?
        }
        if self.has(Button::E) {
            write!(f, "e")?
        }
        Ok(())
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
