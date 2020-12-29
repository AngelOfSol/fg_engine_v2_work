use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Inspect, PartialOrd, Ord,
)]
#[serde(rename_all = "snake_case")]
pub enum AttackId {
    #[serde(rename = "air5a")]
    Air5A,
    #[serde(rename = "air8a")]
    Air8A,
    #[serde(rename = "air5b")]
    Air5B,
    #[serde(rename = "air2b")]
    Air2B,
    #[serde(rename = "attack5a")]
    Attack5A,
    #[serde(rename = "attack2a")]
    Attack2A,
    #[serde(rename = "attack5b")]
    Attack5B,
    #[serde(rename = "attack2b")]
    Attack2B,
    #[serde(rename = "attack3b")]
    Attack3B,
    #[serde(rename = "attack6b")]
    Attack6B,
    MeleeRestitution,
    Butterfly,
}

impl Default for AttackId {
    fn default() -> Self {
        AttackId::Attack5A
    }
}
