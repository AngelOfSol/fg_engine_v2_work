use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum AttackLevel {
    A,
    B,
    C,
    D,
}

impl AttackLevel {
    pub fn hitstun(self) -> i32 {
        match self {
            AttackLevel::A => 13,
            AttackLevel::B => 15,
            AttackLevel::C => 17,
            AttackLevel::D => 19,
        }
    }
}
