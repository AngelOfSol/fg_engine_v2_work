mod cancel_set;
mod flags;
mod global_graphics;
mod hitbox_set;
mod sound_play_info;

mod file;

pub mod components {
    pub use super::cancel_set::*;
    pub use super::flags::*;
    pub use super::global_graphics::*;
    pub use super::hitbox_set::*;
    pub use super::sound_play_info::*;
}
use crate::{game_object::constructors::Constructor, roster::character::typedefs::Character};
use crate::{roster::character::data::Data, timeline::Timeline};
use cancel_set::{CancelSet, StateType};
use flags::Flags;
use hitbox_set::HitboxSet;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use sound_play_info::SoundPlayInfo;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq, Inspect)]
pub struct SpawnerInfo {
    pub frame: usize,
    pub data: Vec<Constructor>,
}

#[derive(Clone, Deserialize, Serialize, Inspect, Default)]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct State<C: Character> {
    #[tab = "Flags"]
    pub flags: Timeline<Flags>,
    #[tab = "Cancels"]
    pub cancels: Timeline<CancelSet>,
    #[tab = "Hitboxes"]
    #[inspect_mut_bounds = "C::Attack: Clone"]
    pub hitboxes: Timeline<HitboxSet<C>>,
    #[tab = "Spawns"]
    pub spawns: Vec<SpawnerInfo>,
    #[tab = "Spawns"]
    pub sounds: Vec<SoundPlayInfo<C::Sound>>,
    #[serde(default)]
    pub state_type: StateType,
    #[inspect_mut_bounds = "C::State: Clone"]
    #[serde(alias = "on_expire_state")]
    pub on_expire: OnExpire<C>,
}

impl<C: Character> State<C> {
    pub fn get(&self, frame: usize) -> StateInstant<'_, C> {
        StateInstant {
            flags: self.flags.get(frame).1,
            cancels: self.cancels.get(frame).1,
            hitboxes: self.hitboxes.get(frame).1,
            on_expire: &self.on_expire,
            sounds: &self.sounds,
            spawns: &self.spawns,
            state_type: self.state_type,
            duration: self.duration(),
            frame,
        }
    }
    pub fn set_duration(&mut self, duration: usize) {
        self.cancels.set_duration(duration);
        self.hitboxes.set_duration(duration);
        self.flags.set_duration(duration);
    }
    pub fn duration(&self) -> usize {
        self.flags.duration()
    }
}

pub struct StateInstant<'a, C: Character> {
    pub flags: &'a Flags,
    pub cancels: &'a CancelSet,
    pub hitboxes: &'a HitboxSet<C>,
    pub spawns: &'a [SpawnerInfo],
    pub sounds: &'a [SoundPlayInfo<C::Sound>],
    pub state_type: StateType,
    pub on_expire: &'a OnExpire<C>,
    pub duration: usize,
    pub frame: usize,
}

impl<'a, C: Character> StateInstant<'a, C> {
    pub fn current_spawns(&self) -> impl Iterator<Item = &SpawnerInfo> {
        self.spawns
            .iter()
            .filter(move |item| item.frame == self.frame)
    }
    pub fn current_sounds(&self) -> impl Iterator<Item = &SoundPlayInfo<C::Sound>> {
        self.sounds
            .iter()
            .filter(move |item| item.frame == self.frame)
    }
}

#[derive(Clone, Deserialize, Serialize, Inspect, Default, PartialEq, Eq, Debug)]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct OnExpire<C: Character> {
    pub state_id: C::State,
    pub frame: usize,
}

impl<C: Character> Display for OnExpire<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.frame, self.state_id)
    }
}

impl<C: Character> PartialEq for State<C> {
    fn eq(&self, rhs: &Self) -> bool {
        self.flags.eq(&rhs.flags)
            && self.cancels.eq(&rhs.cancels)
            && self.hitboxes.eq(&rhs.hitboxes)
            && self.state_type.eq(&rhs.state_type)
            && self.on_expire.eq(&rhs.on_expire)
    }
}
impl<C: Character> Eq for State<C> {}

impl<C: Character> std::fmt::Debug for State<C> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut builder = fmt.debug_struct("State");
        let _ = builder.field("flags", &self.flags);
        let _ = builder.field("cancels", &self.cancels);
        let _ = builder.field("hitboxes", &self.hitboxes);
        let _ = builder.field("state_type", &self.state_type);
        let _ = builder.field("on_expire_state", &self.on_expire);
        builder.finish()
    }
}

impl<C: Character> State<C>
where
    Data<C>: Serialize + for<'de> Deserialize<'de>,
{
    pub fn load_from_json(path: PathBuf) -> Self {
        file::load_from_json(path)
    }

    pub fn save(state: &Self, path: PathBuf) {
        file::save(state, path)
    }
}
