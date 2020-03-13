use super::Button;
use serde::{Deserialize, Serialize};
use std::ops::{BitOr, BitOrAssign};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ButtonSet(u8);

impl From<Button> for ButtonSet {
    fn from(value: Button) -> ButtonSet {
        ButtonSet(value as u8)
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
