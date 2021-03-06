use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter};

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    EnumIter,
    Display,
    EnumCount,
    Serialize,
    Deserialize,
    Eq,
    PartialOrd,
    Ord,
    Inspect,
)]
pub enum RosterCharacter {
    Yuyuko,
}

impl Default for RosterCharacter {
    fn default() -> Self {
        Self::Yuyuko
    }
}
