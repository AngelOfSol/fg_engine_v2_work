use crate::game_match::sounds::{SoundList as SL, SoundPath};
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

pub type SoundList = SL<YuyukoSound>;

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, EnumIter, Display, Inspect,
)]
pub enum YuyukoSound {
    Grunt,
}

impl Into<SoundPath<YuyukoSound>> for YuyukoSound {
    fn into(self) -> SoundPath<YuyukoSound> {
        SoundPath::Local(self)
    }
}

impl Default for YuyukoSound {
    fn default() -> Self {
        Self::Grunt
    }
}
