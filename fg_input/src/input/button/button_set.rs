use crate::notation::button_set;

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
    PartialOrd,
    Ord,
    Default,
)]
pub struct ButtonSet(pub u8);

pub const A: ButtonSet = ButtonSet(0b00001);
pub const B: ButtonSet = ButtonSet(0b00010);
pub const C: ButtonSet = ButtonSet(0b00100);
pub const D: ButtonSet = ButtonSet(0b01000);
pub const E: ButtonSet = ButtonSet(0b10000);

impl ButtonSet {
    pub fn from_id(id: usize) -> Self {
        Self(1 << id)
    }
    pub fn is_superset(self, rhs: ButtonSet) -> bool {
        rhs.0 & self.0 == rhs.0
    }
    pub fn is_empty(self) -> bool {
        self.0 == 0
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
        if self.0 & A.0 == A.0 {
            write!(f, "a")?
        }
        if self.0 & B.0 == B.0 {
            write!(f, "b")?
        }
        if self.0 & C.0 == C.0 {
            write!(f, "c")?
        }
        if self.0 & D.0 == D.0 {
            write!(f, "d")?
        }
        if self.0 & E.0 == E.0 {
            write!(f, "e")?
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::ButtonSet;

    use std::str::FromStr;

    #[test]
    fn single_button() {
        assert_eq!(ButtonSet::from_str("a"), Ok(super::A));
        assert_eq!(ButtonSet::from_str("b"), Ok(super::B));
        assert_eq!(ButtonSet::from_str("c"), Ok(super::C));
        assert_eq!(ButtonSet::from_str("d"), Ok(super::D));
        assert_eq!(ButtonSet::from_str("e"), Ok(super::E));
    }
    #[test]
    fn multi_button() {
        assert_eq!(ButtonSet::from_str("ab"), Ok(super::A | super::B));
        assert_eq!(
            ButtonSet::from_str("abcde"),
            Ok(super::A | super::B | super::C | super::D | super::E)
        );
        assert_eq!(
            ButtonSet::from_str("ace"),
            Ok(super::A | super::C | super::E)
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
