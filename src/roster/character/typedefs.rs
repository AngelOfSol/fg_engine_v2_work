pub mod state;

use super::data::Data;
use crate::character::state::{State, StateInstant};
use hecs::Component;
use state::StateConsts;
use std::hash::Hash;

pub trait Id: Hash + Eq + Component {}

impl<T> Id for T where T: Hash + Eq + Component {}

pub trait Character: Sized + Default {
    type Sound: Id;
    type State: Id + StateConsts;
    type Attack: Id;
    type Graphic: Id;
    type Command: Id + Default;
    type StaticData;

    fn round_start_reset(&mut self, data: &Data<Self>);
}

pub type CharacterState<C: Character> = State<C::State, C::Attack, C::Sound>;
pub type CharacterStateInstant<'a, C: Character> = StateInstant<'a, C::State, C::Attack, C::Sound>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timed<Id> {
    pub time: usize,
    pub id: Id,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HitId<Id> {
    pub hitbox_id: usize,
    pub id: Id,
}
