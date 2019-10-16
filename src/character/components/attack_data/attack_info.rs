use super::AttackLevel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttackInfo {
    pub level: AttackLevel,
}

impl AttackInfo {
    pub fn new() -> Self {
        Self {
            level: AttackLevel::A,
        }
    }
}

impl Default for AttackInfo {
    fn default() -> Self {
        Self::new()
    }
}
