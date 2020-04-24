use crate::game_match::sounds::{ChannelName, SoundPath};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SoundPlayInfo<SoundType> {
    pub name: SoundPath<SoundType>,
    pub channel: ChannelName,
    pub frame: usize,
}

impl<SoundType> SoundPlayInfo<SoundType> {
    pub fn new(name: SoundPath<SoundType>) -> Self {
        Self {
            name,
            channel: ChannelName::System,
            frame: 0,
        }
    }
}
