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

    pub fn into_command(self) -> CommandId {
        match self {
            MoveId::Attack5A => CommandId::Attack5A,
            MoveId::Attack2A => CommandId::Attack2A,
            MoveId::Attack5B => CommandId::Attack5B,
            MoveId::Attack3B => CommandId::Attack3B,
            MoveId::Attack2B => CommandId::Attack2B,
            MoveId::Attack6B => CommandId::Attack6B,
            MoveId::Attack5C => CommandId::Attack5C,
            MoveId::Attack2C => CommandId::Attack2C,
            MoveId::Air5A => CommandId::Air5A,
            MoveId::Air8A => CommandId::Air8A,
            MoveId::Air5B => CommandId::Air5B,
            MoveId::Air2B => CommandId::Air2B,
            MoveId::Air5C => CommandId::Air5C,
            MoveId::Air2C => CommandId::Air2C,
            MoveId::Stand => CommandId::Stand,
            MoveId::ToStand => CommandId::StandUp,
            MoveId::Crouch => CommandId::Crouch,
            MoveId::ToCrouch => CommandId::CrouchDown,
            MoveId::WalkBackward => CommandId::WalkBackward,
            MoveId::WalkForward => CommandId::WalkForward,
            MoveId::ForwardDashStart => CommandId::ForwardDash,
            MoveId::ForwardDashEnd => CommandId::DashSkid,
            MoveId::BackDash => CommandId::BackDash,
            MoveId::Jump => CommandId::Jump,
            MoveId::SuperJump => CommandId::SuperJump,
            MoveId::FlyStart => CommandId::Fly,
            MoveId::BorderEscapeJump => CommandId::BorderEscapeJump,
            MoveId::MeleeRestitution => CommandId::MeleeRestitution,
            x => panic!("found {}, expected valid commandId", x),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Display, Inspect)]
pub enum CommandId {
    Attack5A,
    Attack2A,
    Attack5B,
    Attack3B,
    Attack2B,
    Attack6B,
    Attack5C,
    Attack2C,
    Air5A,
    Air8A,
    Air5B,
    Air2B,
    Air5C,
    Air2C,
    Stand,
    StandUp,
    Crouch,
    CrouchDown,
    WalkBackward,
    WalkForward,
    ForwardDash,
    DashSkid,
    BackDash,
    Jump,
    SuperJump,
    Fly,
    BorderEscapeJump,
    MeleeRestitution,
}
impl Default for CommandId {
    fn default() -> Self {
        CommandId::Stand
    }
}
