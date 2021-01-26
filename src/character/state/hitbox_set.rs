use crate::hitbox::Hitbox;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize, Inspect, Default)]
pub struct AttackData<AttackId> {
    pub id: usize,
    pub boxes: Vec<Hitbox>,
    pub data_id: AttackId,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize, Inspect, Default)]
pub struct HitboxSet<AttackId> {
    pub collision: Hitbox,
    pub hurtbox: Vec<Hitbox>,
    pub hitbox: Option<AttackData<AttackId>>,
}
