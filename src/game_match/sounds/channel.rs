use rodio::source::Source;
use rodio::Device;
use rodio::SpatialSink;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumIter};

use super::{AudioBuffer, GlobalSound, SoundPath, SoundState};

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Display, EnumIter, Serialize, Deserialize)]
pub enum ChannelName {
    Hit,
    Announcer,
    System,
    Attack,
    Movement,
    Voice,
    Projectile,
}

pub struct Channel<LocalPath> {
    sink: SpatialSink,
    state: SoundState<SoundPath<LocalPath>>,
}

impl<LocalPath: std::fmt::Debug> std::fmt::Debug for Channel<LocalPath> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.state)
    }
}

impl<LocalPath> Channel<LocalPath>
where
    LocalPath: std::hash::Hash + std::cmp::Eq + std::fmt::Debug,
    SoundState<SoundPath<LocalPath>>: std::cmp::PartialEq + Clone,
{
    pub fn new(
        state: SoundState<SoundPath<LocalPath>>,
        local: &HashMap<LocalPath, AudioBuffer>,
        global: &HashMap<GlobalSound, AudioBuffer>,
        device: &Device,
        fps: u32,
    ) -> Self {
        let sink = SpatialSink::new(device, [0.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        if let Some(buffer) = state.path.get(local, global) {
            sink.append(skip_samples(state.current_frame, fps, buffer.clone()));
        }
        Self { sink, state }
    }
    pub fn handle(
        &mut self,
        next: Option<&SoundState<SoundPath<LocalPath>>>,
        local: &HashMap<LocalPath, AudioBuffer>,
        global: &HashMap<GlobalSound, AudioBuffer>,
        device: &Device,
        fps: u32,
    ) {
        if next.map(|hit| hit == &self.state).unwrap_or(false) {
            self.sink.pause();
        } else if next
            .map(|hit| !hit.is_continuance(&self.state))
            .unwrap_or(true)
        {
            self.sink.stop();

            if let Some(new_hit) = next {
                if let Some(source) = new_hit.path.get(local, global) {
                    self.sink = SpatialSink::new(
                        device,
                        [0.0, 0.0, 0.0],
                        [-1.0, 0.0, 0.0],
                        [1.0, 0.0, 0.0],
                    );
                    let source = source.clone();
                    let source = skip_samples(new_hit.current_frame, fps, source);

                    self.sink.append(source);
                }
            }
        } else {
            self.sink.play();

            if let Some(new_hit) = next {
                self.state = new_hit.clone();
            }
        }
    }
}

fn skip_samples<Sample: rodio::Sample, S: Source<Item = Sample>>(
    frames: u32,
    fps: u32,
    mut source: S,
) -> S {
    let skip_count = frames * source.sample_rate() * source.channels() as u32 / fps;
    if skip_count > 0 {}
    for _ in 0..skip_count {
        source.next();
    }

    source
}
