use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize, Hash, Inspect)]
pub enum StateType {
    Idle,

    #[serde(alias = "Walk")]
    #[serde(alias = "Jump")]
    #[serde(alias = "HiJump")]
    #[serde(alias = "Dash")]
    #[serde(alias = "Fly")]
    Movement,

    #[serde(alias = "Melee")]
    #[serde(alias = "Magic")]
    #[serde(alias = "MeleeSpecial")]
    #[serde(alias = "MagicSpecial")]
    #[serde(alias = "Super")]
    #[serde(alias = "Followup")]
    Attack,

    Hitstun,

    #[serde(alias = "WrongBlockstun")]
    Blockstun,
}

const ALL_STATE_TYPES: [StateType; 5] = [
    StateType::Idle,
    StateType::Movement,
    StateType::Attack,
    StateType::Hitstun,
    StateType::Blockstun,
];

impl Display for StateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StateType::Idle => "Idle",
                StateType::Movement => "Movement",
                StateType::Attack => "Attack",
                StateType::Hitstun => "Hitstun",
                StateType::Blockstun => "Blockstun",
            }
        )
    }
}
impl StateType {
    pub fn buffer_window(self) -> usize {
        if matches!(self, Self::Attack) {
            16
        } else {
            8
        }
    }
    pub fn all() -> &'static [StateType; 5] {
        &ALL_STATE_TYPES
    }
}

impl Default for StateType {
    fn default() -> Self {
        Self::Idle
    }
}
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
pub struct CancelSet {
    // TODO (HASHSET)
    #[skip]
    pub always: HashSet<CommandType>,
    #[skip]
    pub hit: HashSet<CommandType>,
    #[skip]
    pub block: HashSet<CommandType>,
    #[serde(default)]
    pub self_gatling: bool,
}

impl PartialEq for CancelSet {
    fn eq(&self, rhs: &Self) -> bool {
        self.always.eq(&rhs.always)
            && self.hit.eq(&rhs.hit)
            && self.block.eq(&rhs.block)
            && self.self_gatling == rhs.self_gatling
    }
}
impl Eq for CancelSet {}

impl std::fmt::Debug for CancelSet {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut builder = fmt.debug_struct("CancelSet");
        let _ = builder.field("always", &self.always);
        let _ = builder.field("hit", &self.hit);
        let _ = builder.field("block", &self.block);
        builder.finish()
    }
}

impl CancelSet {
    pub fn new() -> Self {
        Self {
            always: HashSet::new(),
            hit: HashSet::new(),
            block: HashSet::new(),
            self_gatling: false,
        }
    }
}
