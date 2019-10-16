use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AttackId {
    #[serde(rename = "attack5a")]
    Attack5A,
    #[serde(rename = "attack5b")]
    Attack5B,
}

impl Default for AttackId {
    fn default() -> Self {
        AttackId::Attack5A
    }
}
