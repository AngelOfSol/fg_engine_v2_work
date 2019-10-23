use crate::typedefs::HashId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize, Hash)]
pub enum MoveType {
    Idle,
    Walk,
    Jump,
    HiJump,
    Dash,
    Melee,
    Magic,
    MeleeSpecial,
    MagicSpecial,
    Super,
    Followup,
    Fly,
    AirDash,
    AirMelee,
    AirMagic,
    AirMeleeSpecial,
    AirMagicSpecial,
    AirSuper,
    AirFollowup,
    Hitstun,
    Blockstun,
}
const ALL_MOVE_TYPES: [MoveType; 21] = [
    MoveType::Idle,
    MoveType::Walk,
    MoveType::Jump,
    MoveType::HiJump,
    MoveType::Dash,
    MoveType::Melee,
    MoveType::Magic,
    MoveType::MeleeSpecial,
    MoveType::MagicSpecial,
    MoveType::Super,
    MoveType::Followup,
    MoveType::Fly,
    MoveType::AirDash,
    MoveType::AirMelee,
    MoveType::AirMagic,
    MoveType::AirMeleeSpecial,
    MoveType::AirMagicSpecial,
    MoveType::AirSuper,
    MoveType::AirFollowup,
    MoveType::Hitstun,
    MoveType::Blockstun,
];
impl MoveType {
    pub fn all() -> &'static [MoveType; 21] {
        &ALL_MOVE_TYPES
    }

    pub fn is_attack(self) -> bool {
        match self {
            MoveType::Melee
            | MoveType::Magic
            | MoveType::MeleeSpecial
            | MoveType::MagicSpecial
            | MoveType::Super
            | MoveType::Followup
            | MoveType::AirMelee
            | MoveType::AirMagic
            | MoveType::AirMeleeSpecial
            | MoveType::AirMagicSpecial
            | MoveType::AirSuper
            | MoveType::AirFollowup => true,
            MoveType::Hitstun
            | MoveType::Blockstun
            | MoveType::Idle
            | MoveType::Walk
            | MoveType::Jump
            | MoveType::HiJump
            | MoveType::Dash
            | MoveType::Fly
            | MoveType::AirDash => false,
        }
    }
}
impl Display for MoveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MoveType::Idle => "Idle",
                MoveType::Walk => "Walk",
                MoveType::Jump => "Jump",
                MoveType::HiJump => "High Jump",
                MoveType::Dash => "Dash",
                MoveType::Fly => "Fly",
                MoveType::Melee => "Melee",
                MoveType::Magic => "Magic",
                MoveType::MeleeSpecial => "Melee Special",
                MoveType::MagicSpecial => "Magic Special",
                MoveType::Super => "Super",
                MoveType::Followup => "Follow Up",
                MoveType::AirDash => "Air Dash",
                MoveType::AirMelee => "Air Melee",
                MoveType::AirMagic => "Air Magic",
                MoveType::AirMeleeSpecial => "Air Melee Special",
                MoveType::AirMagicSpecial => "Air Magic Special",
                MoveType::AirSuper => "Air Super",
                MoveType::AirFollowup => "Air Followup",
                MoveType::Hitstun => "Hitstun",
                MoveType::Blockstun => "Blockstun",
            }
        )
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct CancelSet<Id>
where
    Id: HashId,
{
    pub always: HashSet<MoveType>,
    pub hit: HashSet<MoveType>,
    pub block: HashSet<MoveType>,
    #[serde(default)]
    pub disallow: HashSet<Id>,
}

impl<Id: HashId> CancelSet<Id> {
    pub fn new() -> Self {
        Self {
            always: HashSet::new(),
            hit: HashSet::new(),
            block: HashSet::new(),
            disallow: HashSet::new(),
        }
    }
}
