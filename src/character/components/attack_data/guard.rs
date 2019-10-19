use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
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
