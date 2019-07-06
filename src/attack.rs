use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Serialize)]
pub enum AttackLevel {
    A,
    B,
    C,
    D,
}