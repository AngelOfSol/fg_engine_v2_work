use super::AudioBuffer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use strum_macros::{Display, EnumIter, EnumString};

#[derive(
    PartialEq, Eq, Copy, Clone, Hash, Display, EnumString, EnumIter, Debug, Serialize, Deserialize,
)]
pub enum GlobalSound {
    Block,
    WrongBlock,
    Hit,
    GuardCrush,
    CounterHit,

    GameStart,
    Round1,
    Round2,
    Round3,
    Round4,
    RoundLast,
    RoundStart,
}
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SoundPath<LocalPath> {
    Local(LocalPath),
    Global(GlobalSound),
}
impl<LocalPath: std::fmt::Display> std::fmt::Display for SoundPath<LocalPath> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local(id) => write!(f, "{}", &id),
            Self::Global(id) => write!(f, "global::{}", &id),
        }
    }
}

impl<LocalPath: std::hash::Hash + std::cmp::Eq> SoundPath<LocalPath> {
    pub fn get<'a>(
        &self,
        local: &'a HashMap<LocalPath, AudioBuffer>,
        global: &'a HashMap<GlobalSound, AudioBuffer>,
    ) -> Option<&'a AudioBuffer> {
        match self {
            SoundPath::Local(key) => local.get(key),
            SoundPath::Global(key) => global.get(key),
        }
    }
}

impl From<String> for SoundPath<String> {
    fn from(sound: String) -> Self {
        Self::Local(sound)
    }
}

impl<T> From<GlobalSound> for SoundPath<T> {
    fn from(sound: GlobalSound) -> Self {
        Self::Global(sound)
    }
}
