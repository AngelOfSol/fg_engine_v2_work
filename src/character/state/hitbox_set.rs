use crate::{hitbox::Hitbox, roster::character::typedefs::Character};
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize, Inspect, Default)]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct AttackData<C: Character> {
    pub id: usize,
    pub boxes: Vec<Hitbox>,
    pub data_id: C::Attack,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Inspect, Default)]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct HitboxSet<C: Character> {
    pub collision: Hitbox,
    pub hurtbox: Vec<Hitbox>,
    pub hitbox: Option<AttackData<C>>,
}
