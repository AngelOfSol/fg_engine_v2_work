use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Hash, Inspect)]
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
