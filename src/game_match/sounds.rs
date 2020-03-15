use ggez::{Context, GameResult};
use rodio::buffer::SamplesBuffer;
use rodio::source::Buffered;
use rodio::source::Source;
use rodio::Device;
use rodio::SpatialSink;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct SoundState<T> {
    name: T,
    current_frame: u32,
}

impl<T: std::cmp::PartialEq> SoundState<T> {
    pub fn is_continuance(&self, prev: &Self) -> bool {
        self.name == prev.name
            && prev
                .current_frame
                .checked_add(1)
                .map(|future_prev| future_prev == self.current_frame)
                .unwrap_or(false)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PlayerSoundState {
    hit: Option<SoundState<HitSoundType>>,
}

impl PlayerSoundState {
    pub fn new() -> Self {
        Self { hit: None }
    }
    pub fn update(&mut self) {
        if let Some(hit) = self.hit.as_mut() {
            hit.current_frame += 1;
        }
    }
    pub fn play_hit_sound(&mut self, name: HitSoundType) {
        self.hit = Some(SoundState {
            current_frame: 0,
            name,
        })
    }
}

pub struct PlayerSoundRenderer {
    prev: PlayerSoundState,
    hit_source: SpatialSink,
}

impl PlayerSoundRenderer {
    pub fn new(device: &Device) -> Self {
        Self {
            prev: PlayerSoundState::new(),
            hit_source: SpatialSink::new(
                device,
                [0.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
            ),
        }
    }

    pub fn render_frame(
        &mut self,
        device: &Device,
        data: &SoundList,
        new: &PlayerSoundState,
        fps: u32,
    ) -> GameResult<()> {
        if new
            .hit
            .and_then(|hit| self.prev.hit.map(|old_hit| !hit.is_continuance(&old_hit)))
            .unwrap_or(true)
        {
            if let Some(_) = self.prev.hit {
                self.hit_source.stop();
            }

            if let Some(new_hit) = new.hit {
                if let Some(source) = data.hits.get(&new_hit.name) {
                    self.hit_source = SpatialSink::new(
                        device,
                        [0.0, 0.0, 0.0],
                        [-1.0, 0.0, 0.0],
                        [1.0, 0.0, 0.0],
                    );
                    let source = source.clone();
                    let source = skip_samples(new_hit.current_frame, fps, source);

                    self.hit_source.append(source);
                }
            }
        }
        self.prev = *new;
        Ok(())
    }
}

fn skip_samples<Sample: rodio::Sample, S: Source<Item = Sample>>(
    frames: u32,
    fps: u32,
    mut source: S,
) -> S {
    let skip_count = frames * source.sample_rate() * source.channels() as u32 / fps;
    if skip_count > 0 {
        dbg!("skipped audio");
    }
    for _ in 0..skip_count {
        source.next();
    }

    source
}

#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug)]
pub enum HitSoundType {
    Block,
}

pub struct SoundList {
    pub hits: HashMap<HitSoundType, Buffered<SamplesBuffer<f32>>>,
}

impl SoundList {
    pub fn new() -> Self {
        SoundList {
            hits: HashMap::new(),
        }
    }
}
