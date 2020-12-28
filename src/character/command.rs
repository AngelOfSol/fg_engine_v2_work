use super::state::components::MoveType;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Inspect)]
pub enum Effect {
    UseAirAction,
    UseMeter(i32),
    RefillSpirit,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Inspect)]
pub enum Requirement<Id> {
    HasAirActions,
    InBlockstun,
    Grounded,
    Airborne,
    NotLockedOut,
    CanCancel(MoveType),
    #[serde(alias = "CanCancelFrom")]
    CancelFrom(Id),
    NoCancelFrom(Id),
    Meter(i32),
    Spirit(i32),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Inspect, Default)]
pub struct Command<Id> {
    pub reqs: Vec<Requirement<Id>>,
    pub effects: Vec<Effect>,
    pub state_id: Id,
    pub frame: usize,
}

impl Default for Effect {
    fn default() -> Self {
        Self::UseAirAction
    }
}
impl<Id> Default for Requirement<Id> {
    fn default() -> Self {
        Self::CanCancel(MoveType::default())
    }
}
