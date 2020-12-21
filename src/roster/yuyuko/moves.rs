use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Display, Inspect)]
#[serde(rename_all = "snake_case")]
pub enum MoveId {
    Stand,
    WalkBackward,
    WalkForward,
    #[serde(rename = "attack5a")]
    Attack5A,
    #[serde(rename = "attack2a")]
    Attack2A,
    #[serde(rename = "attack5b")]
    Attack5B,
    #[serde(rename = "attack3b")]
    Attack3B,
    #[serde(rename = "attack2b")]
    Attack2B,
    #[serde(rename = "attack6b")]
    Attack6B,
    #[serde(rename = "attack5c")]
    Attack5C,
    #[serde(rename = "attack2c")]
    Attack2C,
    #[serde(rename = "air5a")]
    Air5A,
    #[serde(rename = "air8a")]
    Air8A,
    #[serde(rename = "air5b")]
    Air5B,
    #[serde(rename = "air2b")]
    Air2B,
    #[serde(rename = "air5c")]
    Air5C,
    #[serde(rename = "air2c")]
    Air2C,
    Crouch,
    ToCrouch,
    ToStand,
    ForwardDashStart,
    ForwardDash,
    ForwardDashEnd,
    BackDash,
    Jump,
    SuperJump,
    AirIdle,
    Fly,
    FlyStart,
    FlyEnd,
    HitstunStandStart,
    HitstunStandLoop,
    HitstunAirStart,
    HitstunAirMid1,
    HitstunAirMid2,
    HitstunAirLoop,
    BlockstunAirStart,
    BlockstunAirLoop,
    BlockstunCrouchStart,
    BlockstunCrouchLoop,
    BlockstunStandStart,
    BlockstunStandLoop,
    WrongblockCrouchStart,
    WrongblockCrouchLoop,
    WrongblockStandStart,
    WrongblockStandLoop,
    HitGround,
    GetUp,
    BorderEscapeJump,
    MeleeRestitution,
    GuardCrush,
    RoundStart,
    Dead,
}

impl Default for MoveId {
    fn default() -> Self {
        MoveId::Stand
    }
}

impl MoveId {
    pub fn file_name(self) -> String {
        serde_json::to_string(&self)
            .unwrap()
            .trim_matches('\"')
            .to_owned()
    }
}
