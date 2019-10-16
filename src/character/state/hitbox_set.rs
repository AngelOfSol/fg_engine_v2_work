use crate::hitbox::Hitbox;
use crate::typedefs::collision::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct AttackData {
    pub id: usize,
    pub boxes: Vec<Hitbox>,
}
impl AttackData {
    pub fn new() -> Self {
        Self {
            id: 0,
            boxes: vec![],
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct HitboxSet {
    pub collision: Hitbox,
    pub hurtbox: Vec<Hitbox>,
    pub hitbox: Option<AttackData>,
}

impl HitboxSet {
    pub fn new() -> Self {
        Self {
            collision: Hitbox::with_half_size(Vec2::new(1_000, 5_000)),
            hurtbox: vec![],
            hitbox: None,
        }
    }
}
