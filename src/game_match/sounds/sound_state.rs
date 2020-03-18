use super::{ChannelName, SoundPath};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerSoundState<LocalPath> {
    pub channels: HashMap<ChannelName, SoundState<SoundPath<LocalPath>>>,
}

impl<LocalPath> PlayerSoundState<LocalPath> {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }
    pub fn update(&mut self) {
        for value in self.channels.values_mut() {
            value.current_frame += 1;
        }
    }
    pub fn play_sound(&mut self, slot: ChannelName, path: SoundPath<LocalPath>) {
        self.channels.insert(
            slot,
            SoundState {
                current_frame: 0,
                path,
            },
        );
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoundState<T> {
    pub path: T,
    pub current_frame: u32,
}

impl<T: std::cmp::PartialEq> SoundState<T> {
    pub fn is_continuance(&self, prev: &Self) -> bool {
        self.path == prev.path
            && prev
                .current_frame
                .checked_add(1)
                .map(|future_prev| future_prev == self.current_frame)
                .unwrap_or(false)
    }
}
