use rodio::Device;
use std::collections::HashMap;
use strum::IntoEnumIterator;

use super::{AudioBuffer, Channel, ChannelName, GlobalSound, PlayerSoundState};

#[derive(Debug)]
pub struct PlayerSoundRenderer<LocalPath> {
    channels: HashMap<ChannelName, Channel<LocalPath>>,
}

impl<T: Copy + Eq + std::hash::Hash + std::fmt::Debug> PlayerSoundRenderer<T> {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    pub fn render_frame(
        &mut self,
        device: &Device,
        local: &HashMap<T, AudioBuffer>,
        global: &HashMap<GlobalSound, AudioBuffer>,
        new: &PlayerSoundState<T>,
        fps: u32,
    ) {
        for channel in ChannelName::iter() {
            let new = new.channels.get(&channel);
            use std::collections::hash_map::Entry::*;
            match self.channels.entry(channel) {
                Occupied(mut occupied) => {
                    occupied.get_mut().handle(new, local, global, device, fps);
                }
                Vacant(vacant) => {
                    if let Some(new) = new {
                        vacant.insert(Channel::new(new.clone(), local, global, device, fps));
                    }
                }
            };
        }
    }
}
