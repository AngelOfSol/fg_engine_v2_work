use rodio::buffer::SamplesBuffer;
use rodio::source::Buffered;
use rodio::source::Source;
use rodio::Device;
use rodio::SpatialSink;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

pub type AudioBuffer = Buffered<SamplesBuffer<f32>>;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Display, Debug, Serialize, Deserialize)]
pub enum GlobalSound {
    Block,
}
#[derive(PartialEq, Eq, Copy, Clone, Hash, Display, Debug, Serialize, Deserialize)]
pub enum SoundPath<LocalPath> {
    Local(LocalPath),
    Global(GlobalSound),
}

impl<LocalPath: std::hash::Hash + std::cmp::Eq> SoundPath<LocalPath> {
    fn get<'a>(
        &self,
        local: &'a HashMap<LocalPath, AudioBuffer>,
        global: &'a HashMap<GlobalSound, AudioBuffer>,
    ) -> Option<&'a AudioBuffer> {
        match self {
            SoundPath::Local(key) => local.get(key),
            SoundPath::Global(key) => global.get(key),
        }
    }
}

impl<T> From<GlobalSound> for SoundPath<T> {
    fn from(sound: GlobalSound) -> Self {
        Self::Global(sound)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoundState<T> {
    path: T,
    current_frame: u32,
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

struct Channel<LocalPath> {
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
    fn handle(
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerSoundState<LocalPath> {
    channels: HashMap<ChannelName, SoundState<SoundPath<LocalPath>>>,
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

        //self.prev = new.clone();
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

pub struct SoundList {
    pub data: HashMap<GlobalSound, AudioBuffer>,
}

impl SoundList {
    pub fn new() -> Self {
        SoundList {
            data: HashMap::new(),
        }
    }
}
