use crate::roster::generic_character::move_id::GenericMoveId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MoveId {
    Stand,
    WalkBackward,
    WalkForward,
    #[serde(rename = "attack5a")]
    Attack5A,
    #[serde(rename = "attack6b")]
    Attack6B,
    #[serde(rename = "attack5c")]
    Attack5C,
    #[serde(rename = "air5a")]
    Air5A,
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
}

impl Default for MoveId {
    fn default() -> Self {
        MoveId::Stand
    }
}

#[allow(clippy::inherent_to_string)]
impl MoveId {
    pub fn to_string(self) -> String {
        serde_json::to_string(&self)
            .unwrap()
            .trim_matches('\"')
            .to_owned()
    }
}

impl GenericMoveId for MoveId {
    const STARTING_STATE: Self = Self::Stand;
    const HITSTUN_AIR_START: Self = Self::HitstunAirStart;
    const HITSTUN_STAND_START: Self = Self::HitstunStandStart;
    const BLOCKSTUN_AIR_START: Self = Self::BlockstunAirStart;
    const BLOCKSTUN_CROUCH_START: Self = Self::BlockstunCrouchStart;
    const BLOCKSTUN_STAND_START: Self = Self::BlockstunStandStart;
    const WRONGBLOCK_CROUCH_START: Self = Self::WrongblockCrouchStart;
    const WRONGBLOCK_STAND_START: Self = Self::WrongblockStandStart;

    const FLY_START: Self = Self::FlyStart;
    const JUMP: Self = Self::Jump;
    const BORDER_ESCAPE_JUMP: Self = Self::BorderEscapeJump;
    const SUPER_JUMP: Self = Self::SuperJump;
    const CROUCH_IDLE: Self = Self::Crouch;
    const STAND_IDLE: Self = Self::Stand;
    const AIR_IDLE: Self = Self::AirIdle;
    const FLY_CONTINUOUS: Self = Self::Fly;
    const FLY_END: Self = Self::FlyEnd;
    const MELEE_RESTITUTION: Self = Self::MeleeRestitution;
    const KNOCKDOWN_START: Self = Self::HitGround;
}
