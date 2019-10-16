use super::AttackLevel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttackInfo {
    pub level: AttackLevel,
}

impl Default for AttackInfo {
    fn default() -> Self {
        Self {
            level: AttackLevel::A,
        }
    }
}
