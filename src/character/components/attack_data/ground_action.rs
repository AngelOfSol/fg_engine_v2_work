use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum GroundAction {
    Knockdown,
    GroundSlam,
    OnTheGround,
}

impl Default for GroundAction {
    fn default() -> Self {
        Self::Knockdown
    }
}
