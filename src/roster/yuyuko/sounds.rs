use crate::game_match::sounds::SoundPath;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, EnumIter, Display, Inspect,
)]
pub enum SoundId {
    Grunt,
}

impl Into<SoundPath<SoundId>> for SoundId {
    fn into(self) -> SoundPath<SoundId> {
        SoundPath::Local(self)
    }
}

impl Default for SoundId {
    fn default() -> Self {
        Self::Grunt
    }
}
