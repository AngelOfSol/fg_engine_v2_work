use super::{attacks::AttackId, moves::MoveId, sounds::YuyukoSound};
use crate::character::state::{State as StateTemplate, StateInstant as StateInstantTemplate};
use std::collections::HashMap;

pub type State = StateTemplate<MoveId, AttackId, YuyukoSound>;
pub type StateInstant<'a> = StateInstantTemplate<'a, MoveId, AttackId, YuyukoSound>;
pub type StateDataMap = HashMap<MoveId, State>;
