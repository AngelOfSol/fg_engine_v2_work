use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

use crate::hitbox::Hitbox;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, Inspect, Default)]
pub struct Speed(pub i32);

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, Inspect, Default)]
pub struct TotalHits(pub i32);
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Inspect, Default)]
pub struct AttackData {
    pub id: usize,
    pub boxes: Vec<Hitbox>,
}
