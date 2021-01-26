pub mod state;

use super::data::Data;
use hecs::Component;
use inspect_design::{
    traits::{Inspect, InspectMut},
    Inspect,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use state::StateConsts;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};
use strum::IntoEnumIterator;
pub trait Id:
    Hash
    + PartialEq
    + Eq
    + Component
    + Copy
    + Debug
    + for<'de> Deserialize<'de>
    + Serialize
    + Inspect
    + InspectMut
    + Default
    + Display
    + IntoEnumIterator
    + PartialOrd
    + Ord
{
}

impl<T> Id for T where
    T: Hash
        + Eq
        + PartialEq
        + Component
        + Copy
        + Debug
        + for<'de> Deserialize<'de>
        + Serialize
        + Inspect
        + InspectMut
        + Default
        + Display
        + IntoEnumIterator
        + PartialOrd
        + Ord
{
}

pub trait AttackObjectData {}

impl<T> AttackObjectData for T {}

pub trait Character: Sized + Default + Clone + Debug + PartialEq + Eq + 'static {
    type Sound: Id;
    type State: Id + StateConsts;
    type Attack: Id;
    type Graphic: Id;
    type ObjectData: Id;
    type Command: Id + Default;
    type StaticData: Serialize + DeserializeOwned + Default;

    fn round_start_reset(&mut self, data: &Data<Self>);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspect, Serialize, Deserialize)]
pub struct Timed<Id> {
    pub time: usize,
    pub id: Id,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Inspect, Default)]
pub struct HitId<Id> {
    pub hitbox_id: usize,
    pub id: Id,
}
