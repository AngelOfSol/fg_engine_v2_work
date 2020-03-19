use crate::game_match::sounds::ChannelName;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SoundPlayInfo<SoundType> {
    pub name: SoundType,
    pub channel: ChannelName,
    pub frame: usize,
}

impl<SoundType> SoundPlayInfo<SoundType> {
    pub fn new(name: SoundType) -> Self {
        Self {
            name,
            channel: ChannelName::System,
            frame: 0,
        }
    }
}
