mod channel;
mod path;
mod renderer;
mod sound_state;

pub use channel::ChannelName;
pub use path::{GlobalSound, SoundPath};
pub use renderer::PlayerSoundRenderer;
pub use sound_state::PlayerSoundState;

use channel::Channel;
use sound_state::SoundState;

use rodio::buffer::SamplesBuffer;
use rodio::source::Buffered;
use std::collections::HashMap;

pub type AudioBuffer = Buffered<SamplesBuffer<f32>>;

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
