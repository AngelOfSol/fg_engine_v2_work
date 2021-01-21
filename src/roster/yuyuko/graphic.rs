use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(
    Debug,
    Copy,
    Clone,
    Hash,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    EnumIter,
    Display,
    Inspect,
    PartialOrd,
    Ord,
)]
#[serde(rename_all = "snake_case")]
pub enum Graphic {
    SuperJumpParticle,
    HitEffect,
    Butterfly1,
    Butterfly2,
    Butterfly3,
    Butterfly4,
    Ghost,
    Stand,
    WalkBackward,
    WalkForward,
    Attack5A,
    Attack2A,
    Attack5B,
    Attack3B,
    Attack2B,
    Attack6B,
    Attack5C,
    Attack2C,
    Attack5D,
    Attack2D,
    Air5A,
    Air8A,
    Air5B,
    Air2B,
    Air5C,
    Air2C,
    DragonPunch,
    SuperDragonPunch,
    Crouch,
    ToCrouch,
    ToStand,
    ForwardDashStart,
    ForwardDash,
    ForwardDashEnd,
    BackDash,
    Jump,
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
    MeleeRestitution,
    GuardCrush,
    RoundStart,
    Dead,
}

impl Default for Graphic {
    fn default() -> Self {
        Self::Stand
    }
}

impl Graphic {
    pub fn file_name(self) -> String {
        serde_json::to_string(&self)
            .unwrap()
            .trim_matches('\"')
            .to_owned()
    }
}
