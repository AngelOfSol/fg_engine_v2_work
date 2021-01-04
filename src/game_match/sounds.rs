mod channel;
mod path;
mod renderer;
mod sound_state;

pub use channel::ChannelName;
pub use path::{GlobalSound, SoundPath};
pub use renderer::SoundRenderer;
pub use sound_state::PlayerSoundState;

use channel::Channel;
use rodio::buffer::SamplesBuffer;
use rodio::source::Buffered;
use sound_state::SoundState;
use std::collections::HashMap;

pub type AudioBuffer = Buffered<SamplesBuffer<f32>>;
pub type GlobalSoundList = SoundList<GlobalSound>;

#[derive(Clone)]
pub struct SoundList<T> {
    pub data: HashMap<T, AudioBuffer>,
}

impl<T> SoundList<T> {
    pub fn new() -> Self {
        SoundList {
            data: HashMap::new(),
        }
    }
}

impl<T> Default for SoundList<T> {
    fn default() -> Self {
        Self::new()
    }
}
