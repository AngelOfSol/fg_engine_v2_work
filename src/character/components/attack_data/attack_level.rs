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
            AttackLevel::A => 15,
            AttackLevel::B => 17,
            AttackLevel::C => 19,
            AttackLevel::D => 21,
        }
    }
    pub fn blockstun(self) -> i32 {
        match self {
            AttackLevel::A => 13,
            AttackLevel::B => 15,
            AttackLevel::C => 17,
            AttackLevel::D => 19,
        }
    }
    pub fn wrongblockstun(self) -> i32 {
        match self {
            AttackLevel::A => 17,
            AttackLevel::B => 19,
            AttackLevel::C => 21,
            AttackLevel::D => 23,
        }
    }
}
