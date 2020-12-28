use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize, Hash, Inspect)]
pub enum CommandType {
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

impl Default for CommandType {
    fn default() -> Self {
        Self::Idle
    }
}

const ALL_MOVE_TYPES: [CommandType; 22] = [
    CommandType::Idle,
    CommandType::Walk,
    CommandType::Jump,
    CommandType::HiJump,
    CommandType::Dash,
    CommandType::Melee,
    CommandType::Magic,
    CommandType::MeleeSpecial,
    CommandType::MagicSpecial,
    CommandType::Super,
    CommandType::Followup,
    CommandType::Fly,
    CommandType::AirDash,
    CommandType::AirMelee,
    CommandType::AirMagic,
    CommandType::AirMeleeSpecial,
    CommandType::AirMagicSpecial,
    CommandType::AirSuper,
    CommandType::AirFollowup,
    CommandType::Hitstun,
    CommandType::Blockstun,
    CommandType::WrongBlockstun,
];
impl CommandType {
    pub fn all() -> &'static [CommandType; 22] {
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
            CommandType::Melee
            | CommandType::Magic
            | CommandType::MeleeSpecial
            | CommandType::MagicSpecial
            | CommandType::Super
            | CommandType::Followup
            | CommandType::AirMelee
            | CommandType::AirMagic
            | CommandType::AirMeleeSpecial
            | CommandType::AirMagicSpecial
            | CommandType::AirSuper
            | CommandType::AirFollowup => true,
            CommandType::Hitstun
            | CommandType::Blockstun
            | CommandType::WrongBlockstun
            | CommandType::Idle
            | CommandType::Walk
            | CommandType::Jump
            | CommandType::HiJump
            | CommandType::Dash
            | CommandType::Fly
            | CommandType::AirDash => false,
        }
    }
    pub fn is_movement(self) -> bool {
        match self {
            CommandType::Walk
            | CommandType::Jump
            | CommandType::HiJump
            | CommandType::Dash
            | CommandType::Fly
            | CommandType::AirDash => true,
            CommandType::Hitstun
            | CommandType::Melee
            | CommandType::Magic
            | CommandType::MeleeSpecial
            | CommandType::MagicSpecial
            | CommandType::Super
            | CommandType::Followup
            | CommandType::AirMelee
            | CommandType::AirMagic
            | CommandType::AirMeleeSpecial
            | CommandType::AirMagicSpecial
            | CommandType::AirSuper
            | CommandType::AirFollowup
            | CommandType::Blockstun
            | CommandType::WrongBlockstun
            | CommandType::Idle => false,
        }
    }
    pub fn is_stun(self) -> bool {
        matches!(
            self,
            CommandType::Hitstun | CommandType::Blockstun | CommandType::WrongBlockstun
        )
    }
    pub fn is_blockstun(self) -> bool {
        matches!(self, CommandType::Blockstun | CommandType::WrongBlockstun)
    }
}
impl Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CommandType::Idle => "Idle",
                CommandType::Walk => "Walk",
                CommandType::Jump => "Jump",
                CommandType::HiJump => "High Jump",
                CommandType::Dash => "Dash",
                CommandType::Fly => "Fly",
                CommandType::Melee => "Melee",
                CommandType::Magic => "Magic",
                CommandType::MeleeSpecial => "Melee Special",
                CommandType::MagicSpecial => "Magic Special",
                CommandType::Super => "Super",
                CommandType::Followup => "Follow Up",
                CommandType::AirDash => "Air Dash",
                CommandType::AirMelee => "Air Melee",
                CommandType::AirMagic => "Air Magic",
                CommandType::AirMeleeSpecial => "Air Melee Special",
                CommandType::AirMagicSpecial => "Air Magic Special",
                CommandType::AirSuper => "Air Super",
                CommandType::AirFollowup => "Air Followup",
                CommandType::Hitstun => "Hitstun",
                CommandType::Blockstun => "Blockstun",
                CommandType::WrongBlockstun => "Wrong Blockstun",
            }
        )
    }
}
#[derive(Clone, Deserialize, Serialize, Inspect, Default)]
pub struct CancelSet<Id> {
    // TODO (HASHSET)
    #[skip]
    pub always: HashSet<CommandType>,
    #[skip]
    pub hit: HashSet<CommandType>,
    #[skip]
    pub block: HashSet<CommandType>,
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
