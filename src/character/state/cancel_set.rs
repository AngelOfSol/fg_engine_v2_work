use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;

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
    WrongBlockstun,
}
const ALL_MOVE_TYPES: [MoveType; 22] = [
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
    MoveType::WrongBlockstun,
];
impl MoveType {
    pub fn all() -> &'static [MoveType; 22] {
        &ALL_MOVE_TYPES
    }

    pub fn buffer_window(self) -> usize {
        if self.is_attack() {
            16
        } else {
            8
        }
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
            | MoveType::WrongBlockstun
            | MoveType::Idle
            | MoveType::Walk
            | MoveType::Jump
            | MoveType::HiJump
            | MoveType::Dash
            | MoveType::Fly
            | MoveType::AirDash => false,
        }
    }
    pub fn is_movement(self) -> bool {
        match self {
            MoveType::Walk
            | MoveType::Jump
            | MoveType::HiJump
            | MoveType::Dash
            | MoveType::Fly
            | MoveType::AirDash => true,
            MoveType::Hitstun
            | MoveType::Melee
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
            | MoveType::AirFollowup
            | MoveType::Blockstun
            | MoveType::WrongBlockstun
            | MoveType::Idle => false,
        }
    }
    pub fn is_stun(self) -> bool {
        matches!(
            self,
            MoveType::Hitstun | MoveType::Blockstun | MoveType::WrongBlockstun
        )
    }
    pub fn is_blockstun(self) -> bool {
        matches!(self, MoveType::Blockstun | MoveType::WrongBlockstun)
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
                MoveType::WrongBlockstun => "Wrong Blockstun",
            }
        )
    }
}
#[derive(Clone, Deserialize, Serialize, Inspect)]
pub struct CancelSet<Id> {
    #[skip]
    pub always: HashSet<MoveType>,
    #[skip]
    pub hit: HashSet<MoveType>,
    #[skip]
    pub block: HashSet<MoveType>,
    #[serde(bound(
        serialize = "HashSet<Id>: Serialize",
        deserialize = "HashSet<Id>: Deserialize<'de>"
    ))]
    #[serde(default)]
    pub self_gatling: bool,
    #[skip]
    pub disallow: HashSet<Id>,
}

impl<Id> PartialEq for CancelSet<Id>
where
    HashSet<Id>: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.always.eq(&rhs.always)
            && self.hit.eq(&rhs.hit)
            && self.block.eq(&rhs.block)
            && self.disallow.eq(&rhs.disallow)
            && self.self_gatling == rhs.self_gatling
    }
}
impl<Id> Eq for CancelSet<Id> where HashSet<Id>: PartialEq {}

impl<Id> std::fmt::Debug for CancelSet<Id>
where
    HashSet<Id>: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut builder = fmt.debug_struct("CancelSet");
        let _ = builder.field("always", &self.always);
        let _ = builder.field("hit", &self.hit);
        let _ = builder.field("block", &self.block);
        let _ = builder.field("disallow", &self.disallow);
        builder.finish()
    }
}

impl<Id: Eq + Hash> CancelSet<Id> {
    pub fn new() -> Self {
        Self {
            always: HashSet::new(),
            hit: HashSet::new(),
            block: HashSet::new(),
            disallow: HashSet::new(),
            self_gatling: false,
        }
    }
}
