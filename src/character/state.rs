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
use crate::game_object::constructors::Constructor;
use crate::timeline::Timeline;
use cancel_set::{CancelSet, StateType};
use flags::Flags;
use hitbox_set::HitboxSet;
use inspect_design::Inspect;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sound_play_info::SoundPlayInfo;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq, Inspect)]
pub struct SpawnerInfo {
    pub frame: usize,
    pub data: Vec<Constructor>,
}

#[derive(Clone, Deserialize, Serialize, Inspect, Default)]
pub struct State<Id, AttackId, SoundType> {
    #[tab = "Flags"]
    pub flags: Timeline<Flags>,
    #[tab = "Cancels"]
    pub cancels: Timeline<CancelSet>,
    #[inspect_mut_bounds = "AttackId: Clone"]
    #[tab = "Hitboxes"]
    pub hitboxes: Timeline<HitboxSet<AttackId>>,
    #[tab = "Spawns"]
    pub spawns: Vec<SpawnerInfo>,
    #[tab = "Spawns"]
    pub sounds: Vec<SoundPlayInfo<SoundType>>,
    #[serde(default)]
    pub state_type: StateType,
    #[inspect_mut_bounds = "Id: Clone"]
    #[serde(alias = "on_expire_state")]
    pub on_expire: OnExpire<Id>,
}

impl<Id, AttackId, SoundType> State<Id, AttackId, SoundType> {
    pub fn get(&self, frame: usize) -> StateInstant<'_, Id, AttackId, SoundType> {
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

pub struct StateInstant<'a, Id, AttackId, SoundType> {
    pub flags: &'a Flags,
    pub cancels: &'a CancelSet,
    pub hitboxes: &'a HitboxSet<AttackId>,
    pub spawns: &'a [SpawnerInfo],
    pub sounds: &'a [SoundPlayInfo<SoundType>],
    pub state_type: StateType,
    pub on_expire: &'a OnExpire<Id>,
    pub duration: usize,
    pub frame: usize,
}

impl<'a, Id, AttackId, SoundType> StateInstant<'a, Id, AttackId, SoundType> {
    pub fn current_spawns(&self) -> impl Iterator<Item = &SpawnerInfo> {
        self.spawns
            .iter()
            .filter(move |item| item.frame == self.frame)
    }
    pub fn current_sounds(&self) -> impl Iterator<Item = &SoundPlayInfo<SoundType>> {
        self.sounds
            .iter()
            .filter(move |item| item.frame == self.frame)
    }
}

#[derive(Clone, Deserialize, Serialize, Inspect, Default, PartialEq, Eq, Debug)]
pub struct OnExpire<Id> {
    pub state_id: Id,
    pub frame: usize,
}

impl<Id: Display> Display for OnExpire<Id> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.frame, self.state_id)
    }
}

impl<Id> From<Id> for OnExpire<Id> {
    fn from(value: Id) -> Self {
        Self {
            state_id: value,
            frame: 0,
        }
    }
}

impl<Id, AttackId, SoundType> PartialEq for State<Id, AttackId, SoundType>
where
    Id: PartialEq,
    AttackId: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.flags.eq(&rhs.flags)
            && self.cancels.eq(&rhs.cancels)
            && self.hitboxes.eq(&rhs.hitboxes)
            && self.state_type.eq(&rhs.state_type)
            && self.on_expire.eq(&rhs.on_expire)
    }
}
impl<Id, AttackId, SoundType> Eq for State<Id, AttackId, SoundType>
where
    Id: PartialEq,
    AttackId: PartialEq,
{
}

impl<Id, AttackId, SoundType> std::fmt::Debug for State<Id, AttackId, SoundType>
where
    Id: std::fmt::Debug,
    AttackId: std::fmt::Debug,
{
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

pub type EditorCharacterState = State<String, String, String>;
fn default_move_type() -> StateType {
    StateType::Idle
}
impl<
        Id: Serialize + DeserializeOwned + Eq + Hash + Default,
        AttackId: Serialize + DeserializeOwned + Default,
        SoundType: Serialize + DeserializeOwned + Default,
    > State<Id, AttackId, SoundType>
{
    pub fn load_from_json(path: PathBuf) -> Self {
        file::load_from_json(path)
    }

    pub fn save(state: &Self, path: PathBuf) {
        file::save(state, path)
    }
}

impl EditorCharacterState {
    pub fn new() -> Self {
        Self {
            flags: Timeline::with_data(vec![(0, Flags::new())], 1).unwrap(),
            cancels: Timeline::with_data(vec![(0, CancelSet::new())], 1).unwrap(),
            hitboxes: Timeline::with_data(vec![(0, HitboxSet::new())], 1).unwrap(),
            state_type: default_move_type(),
            on_expire: OnExpire::default(),
            spawns: Vec::new(),
            sounds: Vec::new(),
        }
    }
}
