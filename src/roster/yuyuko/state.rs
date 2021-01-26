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
    Display,
    Inspect,
    PartialOrd,
    Ord,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
pub enum State {
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
    #[serde(rename = "attack5d")]
    Attack5D,
    #[serde(rename = "attack2d")]
    Attack2D,
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
    MeleeDragonPunch,
    BulletDragonPunch,
    SuperDragonPunch,
    Crouch,
    ToCrouch,
    ToStand,
    ForwardDashStart,
    ForwardDash,
    ForwardDashEnd,
    BackDash,
    Jump,
    JumpForward,
    JumpBackward,
    SuperJump,
    SuperJumpForward,
    SuperJumpBackward,
    AirIdle,
    Fly,
    FlyEnd,
    FlyForward,
    FlyUp,
    FlyDown,
    FlyUpForward,
    FlyDownForward,
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
    Untech,
    BorderEscape,
    BorderEscapeForward,
    BorderEscapeBackward,
    MeleeRestitution,
    GuardCrush,
    RoundStart,
    Dead,
}

impl Default for State {
    fn default() -> Self {
        State::Stand
    }
}
