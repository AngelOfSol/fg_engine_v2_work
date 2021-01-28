use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Inspect)]
pub enum Guard {
    Low,
    Mid,
    High,
}

impl Default for Guard {
    fn default() -> Self {
        Guard::Mid
    }
}
