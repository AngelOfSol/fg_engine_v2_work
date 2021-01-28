use crate::notation::button_set;

use super::Button;
use inspect_design::Inspect;
use nom::{combinator::eof, sequence::terminated, Finish};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    ops::{BitOr, BitOrAssign},
    str::FromStr,
};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    Inspect,
    Default,
    PartialOrd,
    Ord,
)]
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
        terminated(button_set::parse, eof)(s)
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
        assert_eq!(ButtonSet::from_str("a"), Ok(Button::A.into()));
        assert_eq!(ButtonSet::from_str("b"), Ok(Button::B.into()));
        assert_eq!(ButtonSet::from_str("c"), Ok(Button::C.into()));
        assert_eq!(ButtonSet::from_str("d"), Ok(Button::D.into()));
        assert_eq!(ButtonSet::from_str("e"), Ok(Button::E.into()));
    }
    #[test]
    fn multi_button() {
        assert_eq!(ButtonSet::from_str("ab"), Ok(Button::A | Button::B));
        assert_eq!(
            ButtonSet::from_str("abcde"),
            Ok(Button::A | Button::B | Button::C | Button::D | Button::E)
        );
        assert_eq!(
            ButtonSet::from_str("ace"),
            Ok(Button::A | Button::C | Button::E)
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
