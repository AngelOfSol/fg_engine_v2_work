use super::{attacks::AttackId, commands::CommandId, moves::MoveId, sounds::SoundId};
use crate::{
    character::state::{State as StateTemplate, StateInstant as StateInstantTemplate},
    roster::PlayerState as PlayerStateTemplate,
};
use std::collections::HashMap;

pub type State = StateTemplate<MoveId, AttackId, SoundId>;
pub type StateInstant<'a> = StateInstantTemplate<'a, MoveId, AttackId, SoundId>;
pub type StateDataMap = HashMap<MoveId, State>;
pub type PlayerState = PlayerStateTemplate<MoveId, SoundId, CommandId, AttackId>;
