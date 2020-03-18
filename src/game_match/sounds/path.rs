use super::AudioBuffer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use strum_macros::{Display, EnumIter};

#[derive(PartialEq, Eq, Copy, Clone, Hash, Display, EnumIter, Debug, Serialize, Deserialize)]
pub enum GlobalSound {
    Block,
    WrongBlock,
    Hit,
    GuardCrush,
    CounterHit,
}
#[derive(PartialEq, Eq, Copy, Clone, Hash, Display, Debug, Serialize, Deserialize)]
pub enum SoundPath<LocalPath> {
    Local(LocalPath),
    Global(GlobalSound),
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

impl<T> From<GlobalSound> for SoundPath<T> {
    fn from(sound: GlobalSound) -> Self {
        Self::Global(sound)
    }
}
