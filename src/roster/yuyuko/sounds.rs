use crate::game_match::sounds::SoundPath;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(
    Debug,
    Copy,
    Clone,
    Hash,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    EnumIter,
    Display,
    Inspect,
    PartialOrd,
    Ord,
)]
pub enum Sound {
    Grunt,
}

impl Into<SoundPath<Sound>> for Sound {
    fn into(self) -> SoundPath<Sound> {
        SoundPath::Local(self)
    }
}

impl Default for Sound {
    fn default() -> Self {
        Self::Grunt
    }
}
