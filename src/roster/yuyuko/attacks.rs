use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    Hash,
    PartialEq,
    Eq,
    Inspect,
    PartialOrd,
    Ord,
    Display,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
pub enum Attack {
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
    DragonPunch,
    SuperDragonPunch,
    MeleeRestitution,
    Butterfly,
    Ghost,
}

impl Default for Attack {
    fn default() -> Self {
        Attack::Attack5A
    }
}
