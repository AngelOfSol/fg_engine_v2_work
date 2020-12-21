use crate::game_match::sounds::{ChannelName, SoundPath};
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Inspect, Default)]
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
