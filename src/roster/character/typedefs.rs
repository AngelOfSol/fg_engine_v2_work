pub mod state;

use super::data::Data;
use crate::character::state::{State, StateInstant};
use hecs::Component;
use inspect_design::{
    traits::{Inspect, InspectMut},
    Inspect,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use state::StateConsts;
use std::{fmt::Debug, hash::Hash};
pub trait Id:
    Hash
    + PartialEq
    + Eq
    + Component
    + Copy
    + Debug
    + DeserializeOwned
    + Serialize
    + Inspect
    + InspectMut
    + Default
{
}

impl<T> Id for T where
    T: Hash
        + Eq
        + PartialEq
        + Component
        + Copy
        + Debug
        + DeserializeOwned
        + Serialize
        + Inspect
        + InspectMut
        + Default
{
}

pub trait AttackObjectData {}

impl<T> AttackObjectData for T {}

pub trait Character: Sized + Default + Clone + 'static {
    type Sound: Id;
    type State: Id + StateConsts;
    type Attack: Id;
    type Graphic: Id;
    type ObjectData: Id;
    type Command: Id + Default;
    type StaticData;

    fn round_start_reset(&mut self, data: &Data<Self>);
}

pub type CharacterState<C> =
    State<<C as Character>::State, <C as Character>::Attack, <C as Character>::Sound>;
pub type CharacterStateInstant<'a, C> =
    StateInstant<'a, <C as Character>::State, <C as Character>::Attack, <C as Character>::Sound>;

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
