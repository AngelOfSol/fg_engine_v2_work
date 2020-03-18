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

use ggez::{GameError, GameResult};
use rodio::buffer::SamplesBuffer;
use rodio::source::Buffered;
use rodio::source::Source;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub type AudioBuffer = Buffered<SamplesBuffer<f32>>;

#[derive(Serialize)]
pub struct SoundListInfo<T> {
    #[serde(bound(serialize = "HashMap<T, String>: Serialize",))]
    pub paths: HashMap<T, String>,
}

impl<'de, T> Deserialize<'de> for SoundListInfo<T>
where
    HashMap<T, String>: Deserialize<'de>,
    T: std::hash::Hash + std::cmp::Eq + IntoEnumIterator,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let paths: HashMap<T, String> = HashMap::deserialize(deserializer)?;

        if T::iter().all(|path| paths.contains_key(&path)) {
            Ok(SoundListInfo { paths })
        } else {
            Err(serde::de::Error::custom("Missing paths from sound list."))
        }
    }
}

impl<'de, T> SoundListInfo<T>
where
    HashMap<T, String>: Deserialize<'de>,
    T: std::hash::Hash + std::cmp::Eq + IntoEnumIterator,
{
    pub fn load(self) -> GameResult<SoundList<T>> {
        Ok(SoundList {
            data: self
                .paths
                .into_iter()
                .map(|(sound, path)| -> GameResult<_> {
                    let source = rodio::decoder::Decoder::new(std::io::BufReader::new(
                        std::fs::File::open(&path)?,
                    ))
                    .map_err(|_| {
                        GameError::ResourceLoadError("Attempted to parse sound file.".to_owned())
                    })?;

                    let source = rodio::buffer::SamplesBuffer::new(
                        source.channels(),
                        source.sample_rate(),
                        source.convert_samples().collect::<Vec<_>>(),
                    );

                    Ok((sound, source.buffered()))
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

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
