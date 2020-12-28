use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use strum::Display;

use super::YuyukoGraphic;

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

    pub fn into_graphic_id(self) -> YuyukoGraphic {
        match self {
            MoveId::Stand => YuyukoGraphic::Stand,
            MoveId::WalkBackward => YuyukoGraphic::WalkBackward,
            MoveId::WalkForward => YuyukoGraphic::WalkForward,
            MoveId::Attack5A => YuyukoGraphic::Attack5A,
            MoveId::Attack2A => YuyukoGraphic::Attack2A,
            MoveId::Attack5B => YuyukoGraphic::Attack5B,
            MoveId::Attack3B => YuyukoGraphic::Attack3B,
            MoveId::Attack2B => YuyukoGraphic::Attack2B,
            MoveId::Attack6B => YuyukoGraphic::Attack6B,
            MoveId::Attack5C => YuyukoGraphic::Attack5C,
            MoveId::Attack2C => YuyukoGraphic::Attack2C,
            MoveId::Air5A => YuyukoGraphic::Air5A,
            MoveId::Air8A => YuyukoGraphic::Air8A,
            MoveId::Air5B => YuyukoGraphic::Air5B,
            MoveId::Air2B => YuyukoGraphic::Air2B,
            MoveId::Air5C => YuyukoGraphic::Air5C,
            MoveId::Air2C => YuyukoGraphic::Air2C,
            MoveId::Crouch => YuyukoGraphic::Crouch,
            MoveId::ToCrouch => YuyukoGraphic::ToCrouch,
            MoveId::ToStand => YuyukoGraphic::ToStand,
            MoveId::ForwardDashStart => YuyukoGraphic::ForwardDashStart,
            MoveId::ForwardDash => YuyukoGraphic::ForwardDash,
            MoveId::ForwardDashEnd => YuyukoGraphic::ForwardDashEnd,
            MoveId::BackDash => YuyukoGraphic::BackDash,
            MoveId::Jump => YuyukoGraphic::Jump,
            MoveId::SuperJump => YuyukoGraphic::SuperJump,
            MoveId::AirIdle => YuyukoGraphic::AirIdle,
            MoveId::Fly => YuyukoGraphic::Fly,
            MoveId::FlyStart => YuyukoGraphic::FlyStart,
            MoveId::FlyEnd => YuyukoGraphic::FlyEnd,
            MoveId::HitstunStandStart => YuyukoGraphic::HitstunStandStart,
            MoveId::HitstunStandLoop => YuyukoGraphic::HitstunStandLoop,
            MoveId::HitstunAirStart => YuyukoGraphic::HitstunAirStart,
            MoveId::HitstunAirMid1 => YuyukoGraphic::HitstunAirMid1,
            MoveId::HitstunAirMid2 => YuyukoGraphic::HitstunAirMid2,
            MoveId::HitstunAirLoop => YuyukoGraphic::HitstunAirLoop,
            MoveId::BlockstunAirStart => YuyukoGraphic::BlockstunAirStart,
            MoveId::BlockstunAirLoop => YuyukoGraphic::BlockstunAirLoop,
            MoveId::BlockstunCrouchStart => YuyukoGraphic::BlockstunCrouchStart,
            MoveId::BlockstunCrouchLoop => YuyukoGraphic::BlockstunCrouchLoop,
            MoveId::BlockstunStandStart => YuyukoGraphic::BlockstunStandStart,
            MoveId::BlockstunStandLoop => YuyukoGraphic::BlockstunStandLoop,
            MoveId::WrongblockCrouchStart => YuyukoGraphic::WrongblockCrouchStart,
            MoveId::WrongblockCrouchLoop => YuyukoGraphic::WrongblockCrouchLoop,
            MoveId::WrongblockStandStart => YuyukoGraphic::WrongblockStandStart,
            MoveId::WrongblockStandLoop => YuyukoGraphic::WrongblockStandLoop,
            MoveId::HitGround => YuyukoGraphic::HitGround,
            MoveId::GetUp => YuyukoGraphic::GetUp,
            MoveId::BorderEscapeJump => YuyukoGraphic::BorderEscapeJump,
            MoveId::MeleeRestitution => YuyukoGraphic::MeleeRestitution,
            MoveId::GuardCrush => YuyukoGraphic::GuardCrush,
            MoveId::RoundStart => YuyukoGraphic::RoundStart,
            MoveId::Dead => YuyukoGraphic::Dead,
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
