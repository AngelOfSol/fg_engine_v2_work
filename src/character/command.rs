use crate::roster::character::typedefs::Character;

use super::state::components::CommandType;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Inspect)]
pub enum Effect {
    UseAirAction,
    UseMeter(i32),
    RefillSpirit,
    FlipFacing,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Inspect)]
pub enum Requirement<C: Character> {
    HasAirActions,
    InBlockstun,
    Grounded,
    Airborne,
    NotLockedOut,
    CanCancel(CommandType),
    #[serde(alias = "CanCancelFrom")]
    CancelFrom(C::State),
    NoCancelFrom(C::State),
    Meter(i32),
    Spirit(i32),
    CharacterSpecific(C::Requirement),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Inspect, Default)]
pub struct Command<C: Character> {
    pub reqs: Vec<Requirement<C>>,
    pub effects: Vec<Effect>,
    pub state_id: C::State,
    pub frame: usize,
}

impl Default for Effect {
    fn default() -> Self {
        Self::UseAirAction
    }
}
impl<C: Character> Default for Requirement<C> {
    fn default() -> Self {
        Self::CanCancel(CommandType::default())
    }
}
