use crate::hitbox::Hitbox;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize, Inspect, Default)]
pub struct AttackData<Attack> {
    pub id: usize,
    pub boxes: Vec<Hitbox>,
    pub data_id: Attack,
}

#[derive(Debug, Clone, Serialize, Deserialize, Inspect, PartialEq, Default)]
pub struct HitboxSet<Attack> {
    pub collision: Hitbox,
    pub hurtbox: Vec<Hitbox>,
    pub hitbox: Option<AttackData<Attack>>,
}
