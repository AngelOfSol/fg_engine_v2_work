use crate::graphics::Animation;
use crate::hitbox::Hitbox;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BulletInfo {
    pub animation: Animation,
    pub hitbox: Hitbox,
    pub properties: HashSet<String>,
}

impl BulletInfo {
    pub fn new(key: String) -> Self {
        Self {
            animation: Animation::new(key),
            hitbox: Hitbox::new(),
            properties: HashSet::new(),
        }
    }
}
