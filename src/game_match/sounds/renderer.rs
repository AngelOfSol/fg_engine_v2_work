use rodio::Device;
use std::collections::HashMap;
use strum::IntoEnumIterator;

use super::{AudioBuffer, Channel, ChannelName, GlobalSound, PlayerSoundState, SoundPath};

#[derive(Debug)]
pub struct SoundRenderer<LocalPath> {
    channels: HashMap<ChannelName, Channel<LocalPath>>,
}

impl<T> SoundRenderer<T> {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }
}

impl<T: Copy + Eq + std::hash::Hash + std::fmt::Debug> SoundRenderer<SoundPath<T>> {
    pub fn render_frame(
        &mut self,
        device: &Device,
        local: &HashMap<T, AudioBuffer>,
        global: &HashMap<GlobalSound, AudioBuffer>,
        new: &PlayerSoundState<SoundPath<T>>,
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
                        vacant.insert(Channel::new(*new, local, global, device, fps));
                    }
                }
            };
        }
    }
}

impl SoundRenderer<GlobalSound> {
    pub fn render_frame(
        &mut self,
        device: &Device,
        global: &HashMap<GlobalSound, AudioBuffer>,
        new: &PlayerSoundState<GlobalSound>,
        fps: u32,
    ) {
        for channel in ChannelName::iter() {
            let new = new.channels.get(&channel);
            use std::collections::hash_map::Entry::*;
            match self.channels.entry(channel) {
                Occupied(mut occupied) => {
                    occupied.get_mut().handle(new, global, device, fps);
                }
                Vacant(vacant) => {
                    if let Some(new) = new {
                        vacant.insert(Channel::new_global(*new, global, device, fps));
                    }
                }
            };
        }
    }
}
